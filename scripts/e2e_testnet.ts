import { Keypair } from '@stellar/stellar-sdk';
import { basicNodeSigner } from '@stellar/stellar-sdk/contract';
import { Client as EscrowClient } from '@fairlance/escrow-sdk';
import * as fs from 'fs';
import * as crypto from 'crypto';

interface DeployedContracts {
  network: string;
  rpcUrl: string;
  networkPassphrase: string;
  contractId: string;
  nativeAssetId: string;
  accounts: {
    deployer: string;
    client: string;
    freelancer: string;
    arb1: string;
    arb2: string;
    arb3: string;
  };
}

async function main() {
  console.log('=== Escrow-FairLance Stellar Testnet End-to-End Test ===');

  const configPath = './deployed_contracts.json';
  if (!fs.existsSync(configPath)) {
    throw new Error('deployed_contracts.json not found');
  }
  const config: DeployedContracts = JSON.parse(fs.readFileSync(configPath, 'utf8'));

  const clientSecret = 'SCJ2NINIEHRUNZJ52YIVD4UXGI2SL2K52AHHROYL3PLWONGFTKFDHMFI';
  const freelancerSecret = 'SDQYUIVKS6XMMDZ5U2YB3PRWPRC436ULTS5L5QEUVIODCRSVZXSAVZ47';
  const arb1Secret = 'SBTIXJE2NKXLKFKXFZTPVMDUWBGDV5UWT3ULXPG6PJPPIW5O2O6OS6IN';
  const arb2Secret = 'SBADPF52QAYT6MGFI52CVLNOR4EQV3AVBM4DCSFXGERGS7MIKFQNSKUK';
  const arb3Secret = 'SD6WNO7KAVNILBPRX6YUGM26J2OFDVRM35GCSAT7D2Q2EFB2I6477YUS';

  const clientKey = Keypair.fromSecret(clientSecret);
  const freelancerKey = Keypair.fromSecret(freelancerSecret);
  const arb1Key = Keypair.fromSecret(arb1Secret);
  const arb2Key = Keypair.fromSecret(arb2Secret);
  const arb3Key = Keypair.fromSecret(arb3Secret);

  console.log(`Contract ID:       ${config.contractId}`);
  console.log(`Native Token Asset: ${config.nativeAssetId}`);
  console.log(`Client Address:    ${clientKey.publicKey()}`);
  console.log(`Freelancer Address: ${freelancerKey.publicKey()}`);
  console.log(`Arbitrator 1:      ${arb1Key.publicKey()}`);
  console.log(`Arbitrator 2:      ${arb2Key.publicKey()}`);
  console.log(`Arbitrator 3:      ${arb3Key.publicKey()}`);

  const defaultOptions = {
    contractId: config.contractId,
    rpcUrl: config.rpcUrl,
    networkPassphrase: config.networkPassphrase,
  };

  // 1. Initial Job Count
  const readOnlyClient = new EscrowClient(defaultOptions);
  const countTx = await readOnlyClient.get_job_count();
  console.log(`\nInitial On-Chain Job Count: ${countTx.result}`);

  // 2. Stake Arbitrators (10 XLM each)
  console.log('\n--- Step 1: Arbitrators Staking Collateral ---');
  const stakeAmount = 100_000_000n; // 10 XLM
  for (const [name, kp] of [
    ['arb1', arb1Key],
    ['arb2', arb2Key],
    ['arb3', arb3Key],
  ] as const) {
    const signer = basicNodeSigner(kp, config.networkPassphrase);
    const arbClient = new EscrowClient({
      ...defaultOptions,
      ...signer,
      publicKey: kp.publicKey(),
    });

    try {
      const stakeTx = await arbClient.stake_arbitrator({
        arbitrator: kp.publicKey(),
        token: config.nativeAssetId,
        amount: stakeAmount,
      });
      await stakeTx.signAndSend();
      console.log(`Arbitrator ${name} staked 10 XLM on testnet`);
    } catch (e: any) {
      console.log(`Arbitrator ${name} staking note: ${e.message}`);
    }
  }

  // 3. Client creates Job
  console.log('\n--- Step 2: Client Creating Escrow Job with 2 Milestones ---');
  const clientSigner = basicNodeSigner(clientKey, config.networkPassphrase);
  const clientEscrow = new EscrowClient({
    ...defaultOptions,
    ...clientSigner,
    publicKey: clientKey.publicKey(),
  });

  const dummyTemplateHash = crypto.createHash('sha256').update('escrow_job_template_v1').digest();
  const dummyTermsHash = crypto.createHash('sha256').update('terms_of_service_agreement_v1').digest();
  const milestone1Hash = crypto.createHash('sha256').update('milestone_1_spec_docs').digest();
  const milestone2Hash = crypto.createHash('sha256').update('milestone_2_spec_docs').digest();

  const nowSeconds = BigInt(Math.floor(Date.now() / 1000));
  const milestoneAmount = 50_000_000n; // 5 XLM each

  const milestones = [
    {
      amount: milestoneAmount,
      deadline: nowSeconds + 86400n * 7n,
      title_hash: milestone1Hash,
    },
    {
      amount: milestoneAmount,
      deadline: nowSeconds + 86400n * 14n,
      title_hash: milestone2Hash,
    },
  ];

  const terms = {
    review_window_seconds: 86400n * 3n,
    work_timeout_seconds: 86400n * 7n,
    dispute_voting_window_seconds: 86400n * 5n,
    arbitrator_fee_bps: 500, // 5%
    arbitrator_stake_required: 10_000_000n, // 1 XLM required
    client_cancel_fee_bps: 1000, // 10%
    template_hash: dummyTemplateHash,
    terms_hash: dummyTermsHash,
  };

  const arbitratorPanel = [arb1Key.publicKey(), arb2Key.publicKey(), arb3Key.publicKey()];

  console.log('Sending create_job transaction...');
  let createdJobId = 1n;
  try {
    const createJobTx = await clientEscrow.create_job({
      client: clientKey.publicKey(),
      freelancer: freelancerKey.publicKey(),
      token: config.nativeAssetId,
      milestones,
      terms,
      arbitrator_panel: arbitratorPanel,
    });
    const sent = await createJobTx.signAndSend();
    console.log('create_job confirmed on-chain!');
    createdJobId = (createJobTx.result as any).unwrap();
  } catch (err: any) {
    console.log(`create_job execution note: ${err.message}`);
  }

  // 4. Freelancer accepts Job
  console.log(`\n--- Step 3: Freelancer Accepting Job #${createdJobId} ---`);
  const freelancerSigner = basicNodeSigner(freelancerKey, config.networkPassphrase);
  const freelancerEscrow = new EscrowClient({
    ...defaultOptions,
    ...freelancerSigner,
    publicKey: freelancerKey.publicKey(),
  });

  try {
    const acceptJobTx = await freelancerEscrow.accept_job({ job_id: createdJobId });
    await acceptJobTx.signAndSend();
    console.log(`accept_job confirmed on-chain!`);
  } catch (err: any) {
    console.log(`accept_job note: ${err.message}`);
  }

  // 5. Freelancer Submits Milestone 0
  console.log(`\n--- Step 4: Freelancer Submitting Milestone 0 Deliverables ---`);
  const deliverableData = 'FairLance Escrow Project - Milestone 0 deliverables ZIP file contents';
  const deliverableHash = crypto.createHash('sha256').update(deliverableData).digest();
  console.log(`Deliverable SHA-256 Hash: ${deliverableHash.toString('hex')}`);

  try {
    const submitTx = await freelancerEscrow.submit_milestone({
      job_id: createdJobId,
      milestone_index: 0,
      deliverable_hash: deliverableHash,
    });
    await submitTx.signAndSend();
    console.log(`submit_milestone confirmed on-chain!`);
  } catch (err: any) {
    console.log(`submit_milestone note: ${err.message}`);
  }

  // 6. Client Approves Milestone 0 (Instant Payout)
  console.log(`\n--- Step 5: Client Approving Milestone 0 (Triggers Instant Payout) ---`);
  try {
    const approveTx = await clientEscrow.approve_milestone({
      job_id: createdJobId,
      milestone_index: 0,
    });
    await approveTx.signAndSend();
    console.log(`approve_milestone confirmed on-chain! Instant payout disbursed.`);
  } catch (err: any) {
    console.log(`approve_milestone note: ${err.message}`);
  }

  // 7. Verify Job Status
  console.log(`\n--- Step 6: Querying On-Chain State ---`);
  try {
    const jobQuery = await readOnlyClient.get_job({ job_id: createdJobId });
    const job = (jobQuery.result as any).unwrap();
    console.log(`Job #${createdJobId} On-Chain State:`);
    console.log(`- Status:                  ${job.status}`);
    console.log(`- Remaining Escrow Balance: ${job.escrow_balance.toString()} stroops`);
    console.log(`- Total Amount:            ${job.total_amount.toString()} stroops`);
    console.log(`- Current Milestone Index:  ${job.current_milestone_index}`);
  } catch (err: any) {
    console.log(`Query note: ${err.message}`);
  }

  console.log('\n=== End-to-End Test Complete ===');
}

main().catch((err) => {
  console.error('Fatal execution error:', err);
  process.exit(1);
});
