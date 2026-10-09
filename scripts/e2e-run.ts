import { Keypair } from '@stellar/stellar-sdk';
import { Client as FairLanceClient, networks, JobTerms, MilestoneInit } from '../bindings/src/index.js';
import * as crypto from 'crypto';

const RPC_URL = 'https://soroban-testnet.stellar.org';
const PASSPHRASE = networks.testnet.networkPassphrase;
const CONTRACT_ID = networks.testnet.contractId;
const NATIVE_TOKEN = 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC';

async function fundAccount(publicKey: string): Promise<void> {
  console.log(`Funding account ${publicKey} via Friendbot...`);
  try {
    const res = await fetch(`https://friendbot.stellar.org?addr=${publicKey}`);
    if (!res.ok) {
      console.log(`Friendbot response: ${res.statusText} (account may already be funded)`);
    } else {
      console.log(`Account funded: ${publicKey}`);
    }
  } catch (err) {
    console.warn(`Could not fund via Friendbot: ${err}`);
  }
}

async function main() {
  console.log('=== Starting Escrow-FairLance E2E Testnet Run ===');
  console.log(`Target Contract: ${CONTRACT_ID}`);

  // 1. Generate keypairs for all participants
  const clientKey = Keypair.random();
  const freelancerKey = Keypair.random();
  const arb1Key = Keypair.random();
  const arb2Key = Keypair.random();
  const arb3Key = Keypair.random();

  console.log(`Client:     ${clientKey.publicKey()}`);
  console.log(`Freelancer: ${freelancerKey.publicKey()}`);
  console.log(`Arb1:       ${arb1Key.publicKey()}`);
  console.log(`Arb2:       ${arb2Key.publicKey()}`);
  console.log(`Arb3:       ${arb3Key.publicKey()}`);

  // 2. Fund all accounts via Friendbot
  await Promise.all([
    fundAccount(clientKey.publicKey()),
    fundAccount(freelancerKey.publicKey()),
    fundAccount(arb1Key.publicKey()),
    fundAccount(arb2Key.publicKey()),
    fundAccount(arb3Key.publicKey()),
  ]);

  // Wait 3 seconds for ledger inclusion
  await new Promise((r) => setTimeout(r, 3000));

  // Initialize client with FairLanceClient
  const clientApp = new FairLanceClient({
    networkPassphrase: PASSPHRASE,
    contractId: CONTRACT_ID,
    rpcUrl: RPC_URL,
    publicKey: clientKey.publicKey(),
    ...clientKey,
  });

  console.log('✓ Initialized client connection');
  console.log('E2E setup complete. Ready to drive create_job, accept_job, submit_milestone, and approve_milestone.');
}

main().catch(console.error);
