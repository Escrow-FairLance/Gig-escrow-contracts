use crate::contract::{EscrowFairLance, EscrowFairLanceClient};
use crate::errors::ContractError;
use crate::types::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{StellarAssetClient, Client as TokenClient},
    Address, BytesN, Env, Vec,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_id = e.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(e, &contract_id.address()),
        StellarAssetClient::new(e, &contract_id.address()),
    )
}

#[allow(dead_code)]
struct TestFixture<'a> {
    env: Env,
    admin: Address,
    client_user: Address,
    freelancer: Address,
    arb1: Address,
    arb2: Address,
    arb3: Address,
    token_client: TokenClient<'a>,
    token_admin: StellarAssetClient<'a>,
    contract_client: EscrowFairLanceClient<'a>,
}

fn setup_fixture<'a>() -> TestFixture<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let client_user = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let arb1 = Address::generate(&env);
    let arb2 = Address::generate(&env);
    let arb3 = Address::generate(&env);

    let (token_client, token_admin) = create_token_contract(&env, &admin);

    let contract_id = env.register(EscrowFairLance, ());
    let contract_client = EscrowFairLanceClient::new(&env, &contract_id);

    contract_client.initialize(&admin);

    // Mint tokens to client and arbitrators
    token_admin.mint(&client_user, &100_000_000_000);
    token_admin.mint(&arb1, &10_000_000_000);
    token_admin.mint(&arb2, &10_000_000_000);
    token_admin.mint(&arb3, &10_000_000_000);

    TestFixture {
        env,
        admin,
        client_user,
        freelancer,
        arb1,
        arb2,
        arb3,
        token_client,
        token_admin,
        contract_client,
    }
}

fn sample_terms(env: &Env) -> JobTerms {
    JobTerms {
        review_window_seconds: 86400, // 1 day
        work_timeout_seconds: 604800, // 7 days
        dispute_voting_window_seconds: 259200, // 3 days
        arbitrator_fee_bps: 500, // 5%
        arbitrator_stake_required: 1_000_000, // 0.1 XLM
        client_cancel_fee_bps: 200, // 2%
        template_hash: BytesN::from_array(env, &[1u8; 32]),
        terms_hash: BytesN::from_array(env, &[2u8; 32]),
    }
}

#[test]
fn test_staking_and_unstaking() {
    let f = setup_fixture();
    let stake_amount = 5_000_000;

    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &stake_amount);
    let stake = f.contract_client.get_arbitrator_stake(&f.arb1, &f.token_client.address);
    assert_eq!(stake.staked_amount, stake_amount);
    assert_eq!(stake.active_panels_count, 0);

    // Unstake partial
    f.contract_client.unstake_arbitrator(&f.arb1, &f.token_client.address, &2_000_000);
    let stake2 = f.contract_client.get_arbitrator_stake(&f.arb1, &f.token_client.address);
    assert_eq!(stake2.staked_amount, 3_000_000);
}

#[test]
fn test_create_and_accept_job() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb2, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb3, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[10u8; 32]),
        amount: 20_000_000,
        deadline: f.env.ledger().timestamp() + 100_000,
    });
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[11u8; 32]),
        amount: 30_000_000,
        deadline: f.env.ledger().timestamp() + 200_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());
    panel.push_back(f.arb2.clone());
    panel.push_back(f.arb3.clone());

    let terms = sample_terms(&f.env);
    let client_bal_before = f.token_client.balance(&f.client_user);

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &terms,
        &panel,
    );
    assert_eq!(job_id, 1);

    // Escrow transfer check
    let client_bal_after = f.token_client.balance(&f.client_user);
    assert_eq!(client_bal_before - client_bal_after, 50_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Created);
    assert_eq!(job.escrow_balance, 50_000_000);

    // Accept job
    f.contract_client.accept_job(&job_id);
    let job_accepted = f.contract_client.get_job(&job_id);
    assert_eq!(job_accepted.status, JobStatus::Active);
}

#[test]
fn test_milestone_submission_approval_and_completion() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[20u8; 32]),
        amount: 10_000_000,
        deadline: f.env.ledger().timestamp() + 100_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    // Freelancer submits deliverable
    let deliv_hash = BytesN::from_array(&f.env, &[99u8; 32]);
    f.contract_client.submit_milestone(&job_id, &0, &deliv_hash);

    let m = f.contract_client.get_milestone(&job_id, &0);
    assert_eq!(m.status, MilestoneStatus::Submitted);
    assert_eq!(m.deliverable_hash, Some(deliv_hash));

    // Client approves
    let freelancer_bal_before = f.token_client.balance(&f.freelancer);
    f.contract_client.approve_milestone(&job_id, &0);

    let freelancer_bal_after = f.token_client.balance(&f.freelancer);
    assert_eq!(freelancer_bal_after - freelancer_bal_before, 10_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Completed);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_revisions_and_max_2_limit() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[21u8; 32]),
        amount: 15_000_000,
        deadline: f.env.ledger().timestamp() + 100_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    // Submit 1
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[1u8; 32]));

    // Revision 1
    f.contract_client.request_revision(&job_id, &0, &BytesN::from_array(&f.env, &[101u8; 32]));
    let m = f.contract_client.get_milestone(&job_id, &0);
    assert_eq!(m.status, MilestoneStatus::RevisionRequested);
    assert_eq!(m.revision_count, 1);

    // Resubmit 2
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[2u8; 32]));

    // Revision 2
    f.contract_client.request_revision(&job_id, &0, &BytesN::from_array(&f.env, &[102u8; 32]));
    let m2 = f.contract_client.get_milestone(&job_id, &0);
    assert_eq!(m2.revision_count, 2);

    // Resubmit 3
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[3u8; 32]));

    // Revision 3 should fail (MaxRevisionsExceeded)
    let res = f.contract_client.try_request_revision(&job_id, &0, &BytesN::from_array(&f.env, &[103u8; 32]));
    assert_eq!(res, Err(Ok(ContractError::MaxRevisionsExceeded)));
}

#[test]
fn test_auto_release_after_review_window() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[22u8; 32]),
        amount: 25_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[77u8; 32]));

    // Auto-release before window expires fails
    let res_early = f.contract_client.try_auto_release(&job_id, &0);
    assert_eq!(res_early, Err(Ok(ContractError::ReviewWindowNotElapsed)));

    // Advance time past review window (86400s)
    f.env.ledger().set_timestamp(f.env.ledger().timestamp() + 86500);

    let freelancer_bal_before = f.token_client.balance(&f.freelancer);
    f.contract_client.auto_release(&job_id, &0);
    let freelancer_bal_after = f.token_client.balance(&f.freelancer);
    assert_eq!(freelancer_bal_after - freelancer_bal_before, 25_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Completed);
}

#[test]
fn test_dispute_majority_vote_settlement() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb2, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb3, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[30u8; 32]),
        amount: 100_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());
    panel.push_back(f.arb2.clone());
    panel.push_back(f.arb3.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[88u8; 32]));

    // Client opens dispute
    let reason_hash = BytesN::from_array(&f.env, &[55u8; 32]);
    f.contract_client.open_dispute(&job_id, &0, &f.client_user, &reason_hash);

    let dispute = f.contract_client.get_dispute(&job_id, &0);
    assert_eq!(dispute.status, DisputeStatus::Active);

    // Freelancer submits counter evidence
    let ev_hash = BytesN::from_array(&f.env, &[66u8; 32]);
    f.contract_client.submit_evidence(&job_id, &0, &f.freelancer, &ev_hash);
    let evidence_list = f.contract_client.get_evidence(&job_id, &0);
    assert_eq!(evidence_list.len(), 1);

    // Arbitrators vote: Arb1 and Arb2 vote 70% (7000 bps) to freelancer, Arb3 votes 100% (10000 bps)
    f.contract_client.vote_dispute(&job_id, &0, &f.arb1, &7000);
    f.contract_client.vote_dispute(&job_id, &0, &f.arb2, &7000);
    f.contract_client.vote_dispute(&job_id, &0, &f.arb3, &10000);

    let arb1_bal_before = f.token_client.balance(&f.arb1);
    let arb2_bal_before = f.token_client.balance(&f.arb2);
    let arb3_bal_before = f.token_client.balance(&f.arb3);
    let freelancer_bal_before = f.token_client.balance(&f.freelancer);
    let client_bal_before = f.token_client.balance(&f.client_user);

    f.contract_client.resolve_dispute(&job_id, &0);

    // Dispute resolved at 7000 bps (70% freelancer, 30% client)
    let d_res = f.contract_client.get_dispute(&job_id, &0);
    assert_eq!(d_res.status, DisputeStatus::Resolved);
    assert_eq!(d_res.settled_freelancer_share_bps, 7000);

    // Total fee = 5% of 100_000_000 = 5_000_000, split across 3 arbitrators (1_666_668 + 1_666_666 + 1_666_666)
    let arb1_gain = f.token_client.balance(&f.arb1) - arb1_bal_before;
    let arb2_gain = f.token_client.balance(&f.arb2) - arb2_bal_before;
    let arb3_gain = f.token_client.balance(&f.arb3) - arb3_bal_before;
    assert_eq!(arb1_gain + arb2_gain + arb3_gain, 5_000_000);

    // Remaining principal = 95_000_000
    // Freelancer 70% = 66_500_000
    // Client 30% = 28_500_000
    let f_gain = f.token_client.balance(&f.freelancer) - freelancer_bal_before;
    let c_gain = f.token_client.balance(&f.client_user) - client_bal_before;
    assert_eq!(f_gain, 66_500_000);
    assert_eq!(c_gain, 28_500_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Completed);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_cancel_unaccepted_cancellation_fee() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[40u8; 32]),
        amount: 100_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env), // cancel fee is 2% (200 bps)
        &panel,
    );

    let freelancer_bal_before = f.token_client.balance(&f.freelancer);
    let client_bal_before = f.token_client.balance(&f.client_user);

    f.contract_client.cancel_unaccepted(&job_id);

    // 2% fee to freelancer = 2_000_000
    let f_gain = f.token_client.balance(&f.freelancer) - freelancer_bal_before;
    let c_gain = f.token_client.balance(&f.client_user) - client_bal_before;
    assert_eq!(f_gain, 2_000_000);
    assert_eq!(c_gain, 98_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Cancelled);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_mutual_close() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[50u8; 32]),
        amount: 50_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    // Mutually agree: client gets 20_000_000, freelancer gets 30_000_000
    let freelancer_bal_before = f.token_client.balance(&f.freelancer);
    let client_bal_before = f.token_client.balance(&f.client_user);

    f.contract_client.mutual_close(&job_id, &20_000_000, &30_000_000);

    assert_eq!(f.token_client.balance(&f.freelancer) - freelancer_bal_before, 30_000_000);
    assert_eq!(f.token_client.balance(&f.client_user) - client_bal_before, 20_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Closed);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_reclaim_stalled() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[60u8; 32]),
        amount: 40_000_000,
        deadline: f.env.ledger().timestamp() + 10_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    // Fast-forward past deadline
    f.env.ledger().set_timestamp(f.env.ledger().timestamp() + 15_000);

    let client_bal_before = f.token_client.balance(&f.client_user);
    f.contract_client.reclaim_stalled(&job_id, &0);
    let client_bal_after = f.token_client.balance(&f.client_user);
    assert_eq!(client_bal_after - client_bal_before, 40_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::StalledReclaimed);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_abandon_job() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[70u8; 32]),
        amount: 80_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    let client_bal_before = f.token_client.balance(&f.client_user);
    f.contract_client.abandon_job(&job_id);
    let client_bal_after = f.token_client.balance(&f.client_user);
    assert_eq!(client_bal_after - client_bal_before, 80_000_000);

    let job = f.contract_client.get_job(&job_id);
    assert_eq!(job.status, JobStatus::Abandoned);
    assert_eq!(job.escrow_balance, 0);
}

#[test]
fn test_arbitrator_non_voting_slashing_and_tie_break() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb2, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb3, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[80u8; 32]),
        amount: 100_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());
    panel.push_back(f.arb2.clone());
    panel.push_back(f.arb3.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[12u8; 32]));
    f.contract_client.open_dispute(&job_id, &0, &f.client_user, &BytesN::from_array(&f.env, &[13u8; 32]));

    // Only Arb1 and Arb2 vote: Arb1 votes 0%, Arb2 votes 100% -> Tie
    // Arb3 does not vote
    f.contract_client.vote_dispute(&job_id, &0, &f.arb1, &0);
    f.contract_client.vote_dispute(&job_id, &0, &f.arb2, &10000);

    // Fast forward past voting deadline (259200s)
    f.env.ledger().set_timestamp(f.env.ledger().timestamp() + 260_000);

    f.contract_client.resolve_dispute(&job_id, &0);

    // Dispute resolved at 50/50 tie break (5000 bps)
    let d = f.contract_client.get_dispute(&job_id, &0);
    assert_eq!(d.status, DisputeStatus::Resolved);
    assert_eq!(d.settled_freelancer_share_bps, 5000);

    // Arb3 was slashed for non-voting (10% of 1_000_000 required stake = 100_000)
    let arb3_stake = f.contract_client.get_arbitrator_stake(&f.arb3, &f.token_client.address);
    assert_eq!(arb3_stake.slashed_amount, 100_000);
    assert_eq!(arb3_stake.staked_amount, 4_900_000);

    let arb3_stats = f.contract_client.get_arbitrator_stats(&f.arb3);
    assert_eq!(arb3_stats.total_cases_slashed, 1);
}

#[test]
fn test_admin_slash_arbitrator() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let admin_bal_before = f.token_client.balance(&f.admin);
    f.contract_client.slash_arbitrator(
        &f.admin,
        &f.arb1,
        &f.token_client.address,
        &500_000,
        &(SlashingReason::BiasedOutlier as u32),
    );

    let arb1_stake = f.contract_client.get_arbitrator_stake(&f.arb1, &f.token_client.address);
    assert_eq!(arb1_stake.staked_amount, 4_500_000);
    assert_eq!(arb1_stake.slashed_amount, 500_000);

    let admin_bal_after = f.token_client.balance(&f.admin);
    assert_eq!(admin_bal_after - admin_bal_before, 500_000);
}

#[test]
fn test_panel_validation_odd_size_and_uniqueness() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);
    f.contract_client.stake_arbitrator(&f.arb2, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[90u8; 32]),
        amount: 10_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });

    // Even panel (2 arbitrators) should fail
    let mut even_panel = Vec::new(&f.env);
    even_panel.push_back(f.arb1.clone());
    even_panel.push_back(f.arb2.clone());

    let res_even = f.contract_client.try_create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &even_panel,
    );
    assert_eq!(res_even, Err(Ok(ContractError::InvalidArbitratorPanel)));

    // Duplicate arbitrator panel should fail
    let mut dup_panel = Vec::new(&f.env);
    dup_panel.push_back(f.arb1.clone());
    dup_panel.push_back(f.arb1.clone());
    dup_panel.push_back(f.arb2.clone());

    let res_dup = f.contract_client.try_create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &dup_panel,
    );
    assert_eq!(res_dup, Err(Ok(ContractError::InvalidArbitratorPanel)));
}

#[test]
fn test_property_escrow_accounting_balance_invariant() {
    let f = setup_fixture();
    f.contract_client.stake_arbitrator(&f.arb1, &f.token_client.address, &5_000_000);

    let mut milestones = Vec::new(&f.env);
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[1u8; 32]),
        amount: 10_000_000,
        deadline: f.env.ledger().timestamp() + 500_000,
    });
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[2u8; 32]),
        amount: 25_000_000,
        deadline: f.env.ledger().timestamp() + 600_000,
    });
    milestones.push_back(MilestoneInit {
        title_hash: BytesN::from_array(&f.env, &[3u8; 32]),
        amount: 40_000_000,
        deadline: f.env.ledger().timestamp() + 700_000,
    });

    let mut panel = Vec::new(&f.env);
    panel.push_back(f.arb1.clone());

    let job_id = f.contract_client.create_job(
        &f.client_user,
        &f.freelancer,
        &f.token_client.address,
        &milestones,
        &sample_terms(&f.env),
        &panel,
    );
    f.contract_client.accept_job(&job_id);

    // Initial escrow balance == sum of milestones
    let job0 = f.contract_client.get_job(&job_id);
    assert_eq!(job0.escrow_balance, 75_000_000);

    // Approve milestone 0
    f.contract_client.submit_milestone(&job_id, &0, &BytesN::from_array(&f.env, &[11u8; 32]));
    f.contract_client.approve_milestone(&job_id, &0);
    let job1 = f.contract_client.get_job(&job_id);
    assert_eq!(job1.escrow_balance, 65_000_000);

    // Approve milestone 1
    f.contract_client.submit_milestone(&job_id, &1, &BytesN::from_array(&f.env, &[22u8; 32]));
    f.contract_client.approve_milestone(&job_id, &1);
    let job2 = f.contract_client.get_job(&job_id);
    assert_eq!(job2.escrow_balance, 40_000_000);

    // Approve milestone 2 -> Completed, balance 0
    f.contract_client.submit_milestone(&job_id, &2, &BytesN::from_array(&f.env, &[33u8; 32]));
    f.contract_client.approve_milestone(&job_id, &2);
    let job3 = f.contract_client.get_job(&job_id);
    assert_eq!(job3.escrow_balance, 0);
    assert_eq!(job3.status, JobStatus::Completed);
}
