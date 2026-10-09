use soroban_sdk::{symbol_short, Address, BytesN, Env, Symbol};

pub fn emit_job_created(
    env: &Env,
    job_id: u64,
    client: &Address,
    freelancer: &Address,
    token: &Address,
    total_amount: i128,
) {
    env.events().publish(
        (symbol_short!("job"), symbol_short!("created")),
        (job_id, client.clone(), freelancer.clone(), token.clone(), total_amount),
    );
}

pub fn emit_job_accepted(env: &Env, job_id: u64, freelancer: &Address) {
    env.events().publish(
        (symbol_short!("accepted"), job_id),
        freelancer.clone(),
    );
}

pub fn emit_job_declined(env: &Env, job_id: u64, freelancer: &Address) {
    env.events().publish(
        (symbol_short!("declined"), job_id),
        freelancer.clone(),
    );
}

pub fn emit_job_cancelled(
    env: &Env,
    job_id: u64,
    client: &Address,
    refund_amount: i128,
    fee_amount: i128,
) {
    env.events().publish(
        (symbol_short!("cancelled"), job_id),
        (client.clone(), refund_amount, fee_amount),
    );
}

pub fn emit_milestone_submitted(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    deliverable_hash: BytesN<32>,
) {
    env.events().publish(
        (symbol_short!("submitted"), job_id, milestone_index),
        deliverable_hash,
    );
}

pub fn emit_milestone_approved(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    payout_amount: i128,
) {
    env.events().publish(
        (symbol_short!("approved"), job_id, milestone_index),
        payout_amount,
    );
}

pub fn emit_revision_requested(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    revision_count: u32,
    feedback_hash: BytesN<32>,
) {
    env.events().publish(
        (symbol_short!("revision"), job_id, milestone_index),
        (revision_count, feedback_hash),
    );
}

pub fn emit_auto_released(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    payout_amount: i128,
) {
    env.events().publish(
        (symbol_short!("autorel"), job_id, milestone_index),
        payout_amount,
    );
}

pub fn emit_dispute_opened(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    opened_by: &Address,
    reason_hash: BytesN<32>,
) {
    env.events().publish(
        (symbol_short!("dispute"), job_id, milestone_index),
        (opened_by.clone(), reason_hash),
    );
}

pub fn emit_evidence_submitted(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    submitter: &Address,
    evidence_hash: BytesN<32>,
) {
    env.events().publish(
        (symbol_short!("evidence"), job_id, milestone_index),
        (submitter.clone(), evidence_hash),
    );
}

pub fn emit_dispute_voted(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    arbitrator: &Address,
    freelancer_share_bps: u32,
) {
    env.events().publish(
        (symbol_short!("vote"), job_id, milestone_index),
        (arbitrator.clone(), freelancer_share_bps),
    );
}

pub fn emit_dispute_resolved(
    env: &Env,
    job_id: u64,
    milestone_index: u32,
    settled_freelancer_share_bps: u32,
    arbitrator_fee: i128,
) {
    env.events().publish(
        (symbol_short!("resolved"), job_id, milestone_index),
        (settled_freelancer_share_bps, arbitrator_fee),
    );
}

pub fn emit_job_closed(
    env: &Env,
    job_id: u64,
    client_amount: i128,
    freelancer_amount: i128,
) {
    env.events().publish(
        (symbol_short!("closed"), job_id),
        (client_amount, freelancer_amount),
    );
}

pub fn emit_job_completed(env: &Env, job_id: u64) {
    env.events().publish(
        (symbol_short!("complete"), job_id),
        job_id,
    );
}

pub fn emit_job_abandoned(
    env: &Env,
    job_id: u64,
    freelancer: &Address,
    refund_amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "abandoned"), job_id),
        (freelancer.clone(), refund_amount),
    );
}

pub fn emit_stalled_reclaimed(
    env: &Env,
    job_id: u64,
    client: &Address,
    refund_amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "reclaimed"), job_id),
        (client.clone(), refund_amount),
    );
}

pub fn emit_arbitrator_staked(
    env: &Env,
    arbitrator: &Address,
    token: &Address,
    amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "staked"), arbitrator.clone()),
        (token.clone(), amount),
    );
}

pub fn emit_arbitrator_unstaked(
    env: &Env,
    arbitrator: &Address,
    token: &Address,
    amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "unstaked"), arbitrator.clone()),
        (token.clone(), amount),
    );
}

pub fn emit_arbitrator_slashed(
    env: &Env,
    arbitrator: &Address,
    token: &Address,
    amount: i128,
    reason: u32,
) {
    env.events().publish(
        (Symbol::new(env, "slashed"), arbitrator.clone()),
        (token.clone(), amount, reason),
    );
}
