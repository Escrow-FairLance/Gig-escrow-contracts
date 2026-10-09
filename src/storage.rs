use crate::errors::ContractError;
use crate::types::{
    ArbitratorStake, ArbitratorStats, ArbitratorVote, DataKey, Dispute, EvidenceRecord, Job,
    Milestone,
};
use soroban_sdk::{Address, Env, Vec};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400; // ~30 days
const INSTANCE_LIFETIME_THRESHOLD: u32 = 172_800; // ~10 days
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 172_800;

pub fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn bump_persistent(env: &Env, key: &DataKey) {
    if env.storage().persistent().has(key) {
        env.storage()
            .persistent()
            .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }
}

pub fn get_admin(env: &Env) -> Result<Address, ContractError> {
    bump_instance(env);
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    bump_instance(env);
}

pub fn get_job_count(env: &Env) -> u64 {
    bump_instance(env);
    env.storage().instance().get(&DataKey::JobCount).unwrap_or(0)
}

pub fn set_job_count(env: &Env, count: u64) {
    env.storage().instance().set(&DataKey::JobCount, &count);
    bump_instance(env);
}

pub fn get_job(env: &Env, job_id: u64) -> Result<Job, ContractError> {
    let key = DataKey::Job(job_id);
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::JobNotFound)
}

pub fn set_job(env: &Env, job: &Job) {
    let key = DataKey::Job(job.job_id);
    env.storage().persistent().set(&key, job);
    bump_persistent(env, &key);
}

pub fn get_milestone(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
) -> Result<Milestone, ContractError> {
    let key = DataKey::Milestone(job_id, milestone_index);
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::MilestoneNotFound)
}

pub fn set_milestone(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    milestone: &Milestone,
) {
    let key = DataKey::Milestone(job_id, milestone_index);
    env.storage().persistent().set(&key, milestone);
    bump_persistent(env, &key);
}

pub fn get_dispute(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
) -> Result<Dispute, ContractError> {
    let key = DataKey::Dispute(job_id, milestone_index);
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::DisputeNotFound)
}

pub fn set_dispute(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    dispute: &Dispute,
) {
    let key = DataKey::Dispute(job_id, milestone_index);
    env.storage().persistent().set(&key, dispute);
    bump_persistent(env, &key);
}

pub fn get_vote(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    arbitrator: &Address,
) -> Option<ArbitratorVote> {
    let key = DataKey::Vote(job_id, milestone_index, arbitrator.clone());
    bump_persistent(env, &key);
    env.storage().persistent().get(&key)
}

pub fn set_vote(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    vote: &ArbitratorVote,
) {
    let key = DataKey::Vote(job_id, milestone_index, vote.arbitrator.clone());
    env.storage().persistent().set(&key, vote);
    bump_persistent(env, &key);
}

pub fn get_voter_list(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
) -> Vec<Address> {
    let key = DataKey::VoterList(job_id, milestone_index);
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn set_voter_list(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    voters: &Vec<Address>,
) {
    let key = DataKey::VoterList(job_id, milestone_index);
    env.storage().persistent().set(&key, voters);
    bump_persistent(env, &key);
}

pub fn get_evidence_list(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
) -> Vec<EvidenceRecord> {
    let key = DataKey::EvidenceList(job_id, milestone_index);
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn add_evidence(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    record: &EvidenceRecord,
) {
    let mut list = get_evidence_list(env, job_id, milestone_index);
    list.push_back(record.clone());
    let key = DataKey::EvidenceList(job_id, milestone_index);
    env.storage().persistent().set(&key, &list);
    bump_persistent(env, &key);
}

pub fn get_arbitrator_stake(
    env: &Env,
    arbitrator: &Address,
    token: &Address,
) -> ArbitratorStake {
    let key = DataKey::ArbitratorStake(arbitrator.clone(), token.clone());
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(ArbitratorStake {
            staked_amount: 0,
            slashed_amount: 0,
            active_panels_count: 0,
        })
}

pub fn set_arbitrator_stake(
    env: &Env,
    arbitrator: &Address,
    token: &Address,
    stake: &ArbitratorStake,
) {
    let key = DataKey::ArbitratorStake(arbitrator.clone(), token.clone());
    env.storage().persistent().set(&key, stake);
    bump_persistent(env, &key);
}

pub fn get_arbitrator_stats(
    env: &Env,
    arbitrator: &Address,
) -> ArbitratorStats {
    let key = DataKey::ArbitratorStats(arbitrator.clone());
    bump_persistent(env, &key);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(ArbitratorStats {
            total_cases_assigned: 0,
            total_cases_voted: 0,
            total_cases_slashed: 0,
            total_fees_earned: 0,
        })
}

pub fn set_arbitrator_stats(
    env: &Env,
    arbitrator: &Address,
    stats: &ArbitratorStats,
) {
    let key = DataKey::ArbitratorStats(arbitrator.clone());
    env.storage().persistent().set(&key, stats);
    bump_persistent(env, &key);
}
