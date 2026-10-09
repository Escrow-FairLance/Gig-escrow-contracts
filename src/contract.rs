use crate::errors::ContractError;
use crate::events::*;
use crate::storage::*;
use crate::types::*;
use soroban_sdk::{
    contract, contractimpl, token, Address, BytesN, Env, Vec,
};

#[contract]
pub struct EscrowFairLance;

#[contractimpl]
impl EscrowFairLance {
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        set_admin(&env, &admin);
        Ok(())
    }

    /// Stake tokens as an arbitrator to be eligible for selection in panels
    pub fn stake_arbitrator(
        env: Env,
        arbitrator: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        arbitrator.require_auth();
        if amount <= 0 {
            return Err(ContractError::ZeroAmount);
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&arbitrator, &env.current_contract_address(), &amount);

        let mut stake = get_arbitrator_stake(&env, &arbitrator, &token);
        stake.staked_amount = stake
            .staked_amount
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        set_arbitrator_stake(&env, &arbitrator, &token, &stake);

        emit_arbitrator_staked(&env, &arbitrator, &token, amount);
        Ok(())
    }

    /// Withdraw staked arbitrator collateral if no active cases or surplus exists
    pub fn unstake_arbitrator(
        env: Env,
        arbitrator: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        arbitrator.require_auth();
        if amount <= 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut stake = get_arbitrator_stake(&env, &arbitrator, &token);
        if stake.active_panels_count > 0 {
            return Err(ContractError::ActiveCasesPending);
        }
        if stake.staked_amount < amount {
            return Err(ContractError::ArbitratorInsufficientStake);
        }

        stake.staked_amount = stake
            .staked_amount
            .checked_sub(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        set_arbitrator_stake(&env, &arbitrator, &token, &stake);

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &arbitrator, &amount);

        emit_arbitrator_unstaked(&env, &arbitrator, &token, amount);
        Ok(())
    }

    /// Create a new escrow job, funding the full milestone amount
    pub fn create_job(
        env: Env,
        client: Address,
        freelancer: Address,
        token: Address,
        milestones: Vec<MilestoneInit>,
        terms: JobTerms,
        arbitrator_panel: Vec<Address>,
    ) -> Result<u64, ContractError> {
        client.require_auth();

        if client == freelancer {
            return Err(ContractError::CannotSelfArbitrate);
        }
        let m_count = milestones.len();
        if m_count == 0 || m_count > 50 {
            return Err(ContractError::InvalidMilestoneCount);
        }

        // Validate arbitrator panel: odd size between 1 and 7, distinct, non-party
        let panel_size = arbitrator_panel.len();
        if panel_size == 0 || panel_size > 7 || panel_size % 2 == 0 {
            return Err(ContractError::InvalidArbitratorPanel);
        }

        for i in 0..panel_size {
            let arb_i = arbitrator_panel.get(i).unwrap();
            if arb_i == client || arb_i == freelancer {
                return Err(ContractError::CannotSelfArbitrate);
            }
            // Ensure unique arbitrators
            for j in (i + 1)..panel_size {
                if arb_i == arbitrator_panel.get(j).unwrap() {
                    return Err(ContractError::InvalidArbitratorPanel);
                }
            }
            // Check required stake
            if terms.arbitrator_stake_required > 0 {
                let stake = get_arbitrator_stake(&env, &arb_i, &token);
                if stake.staked_amount < terms.arbitrator_stake_required {
                    return Err(ContractError::ArbitratorInsufficientStake);
                }
            }
        }

        // Validate percentage limits
        if terms.arbitrator_fee_bps > 3000 || terms.client_cancel_fee_bps > 5000 {
            return Err(ContractError::InvalidPercentage);
        }

        let now = env.ledger().timestamp();
        let mut total_amount: i128 = 0;

        for i in 0..m_count {
            let m_init = milestones.get(i).unwrap();
            if m_init.amount <= 0 {
                return Err(ContractError::InvalidMilestoneAmount);
            }
            if m_init.deadline <= now {
                return Err(ContractError::InvalidMilestoneDeadline);
            }
            total_amount = total_amount
                .checked_add(m_init.amount)
                .ok_or(ContractError::ArithmeticOverflow)?;
        }

        // Escrow funds transfer from client to contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, &env.current_contract_address(), &total_amount);

        let job_id = get_job_count(&env).checked_add(1).ok_or(ContractError::ArithmeticOverflow)?;
        set_job_count(&env, job_id);

        // Store milestones
        for i in 0..m_count {
            let m_init = milestones.get(i).unwrap();
            let milestone = Milestone {
                title_hash: m_init.title_hash,
                amount: m_init.amount,
                deadline: m_init.deadline,
                status: MilestoneStatus::Pending,
                deliverable_hash: None,
                submitted_at: 0,
                revision_count: 0,
                revision_feedback_hash: None,
            };
            set_milestone(&env, job_id, i, &milestone);
        }

        // Increment active panels count for arbitrators
        for i in 0..panel_size {
            let arb = arbitrator_panel.get(i).unwrap();
            let mut stake = get_arbitrator_stake(&env, &arb, &token);
            stake.active_panels_count = stake.active_panels_count.saturating_add(1);
            set_arbitrator_stake(&env, &arb, &token, &stake);

            let mut stats = get_arbitrator_stats(&env, &arb);
            stats.total_cases_assigned = stats.total_cases_assigned.saturating_add(1);
            set_arbitrator_stats(&env, &arb, &stats);
        }

        let job = Job {
            job_id,
            client: client.clone(),
            freelancer: freelancer.clone(),
            token: token.clone(),
            terms,
            total_amount,
            escrow_balance: total_amount,
            arbitrator_panel,
            status: JobStatus::Created,
            milestone_count: m_count,
            current_milestone_index: 0,
            created_at: now,
        };
        set_job(&env, &job);

        emit_job_created(&env, job_id, &client, &freelancer, &token, total_amount);
        Ok(job_id)
    }

    /// Freelancer accepts the job offer
    pub fn accept_job(env: Env, job_id: u64) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.freelancer.require_auth();

        if job.status != JobStatus::Created {
            return Err(ContractError::InvalidJobStatus);
        }

        job.status = JobStatus::Active;
        set_job(&env, &job);

        emit_job_accepted(&env, job_id, &job.freelancer);
        Ok(())
    }

    /// Freelancer declines the job offer, issuing a 100% refund to the client
    pub fn decline_job(env: Env, job_id: u64) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.freelancer.require_auth();

        if job.status != JobStatus::Created {
            return Err(ContractError::InvalidJobStatus);
        }

        let refund_amount = job.escrow_balance;
        job.escrow_balance = 0;
        job.status = JobStatus::Declined;
        set_job(&env, &job);

        decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);

        let token_client = token::Client::new(&env, &job.token);
        token_client.transfer(&env.current_contract_address(), &job.client, &refund_amount);

        emit_job_declined(&env, job_id, &job.freelancer);
        Ok(())
    }

    /// Client cancels job before freelancer acceptance (applies cancellation fee if set)
    pub fn cancel_unaccepted(env: Env, job_id: u64) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.client.require_auth();

        if job.status != JobStatus::Created {
            return Err(ContractError::InvalidJobStatus);
        }

        let total = job.escrow_balance;
        let fee_amount = if job.terms.client_cancel_fee_bps > 0 {
            (total * (job.terms.client_cancel_fee_bps as i128)) / 10000
        } else {
            0
        };
        let refund_amount = total - fee_amount;

        job.escrow_balance = 0;
        job.status = JobStatus::Cancelled;
        set_job(&env, &job);

        decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);

        let token_client = token::Client::new(&env, &job.token);
        if fee_amount > 0 {
            token_client.transfer(&env.current_contract_address(), &job.freelancer, &fee_amount);
        }
        token_client.transfer(&env.current_contract_address(), &job.client, &refund_amount);

        emit_job_cancelled(&env, job_id, &job.client, refund_amount, fee_amount);
        Ok(())
    }

    /// Freelancer submits milestone deliverables (hash of files)
    pub fn submit_milestone(
        env: Env,
        job_id: u64,
        milestone_index: u32,
        deliverable_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        let job = get_job(&env, job_id)?;
        job.freelancer.require_auth();

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Pending
            && milestone.status != MilestoneStatus::RevisionRequested
        {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let now = env.ledger().timestamp();
        if now > milestone.deadline {
            return Err(ContractError::DeadlinePassed);
        }

        milestone.deliverable_hash = Some(deliverable_hash.clone());
        milestone.submitted_at = now;
        milestone.status = MilestoneStatus::Submitted;
        set_milestone(&env, job_id, milestone_index, &milestone);

        emit_milestone_submitted(&env, job_id, milestone_index, deliverable_hash);
        Ok(())
    }

    /// Client approves milestone deliverable, triggering instant payout
    pub fn approve_milestone(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.client.require_auth();

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Submitted {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let payout_amount = milestone.amount;
        milestone.status = MilestoneStatus::Approved;
        set_milestone(&env, job_id, milestone_index, &milestone);

        job.escrow_balance = job
            .escrow_balance
            .checked_sub(payout_amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        job.current_milestone_index = job.current_milestone_index.saturating_add(1);

        let is_completed = job.current_milestone_index >= job.milestone_count;
        if is_completed {
            job.status = JobStatus::Completed;
            decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);
        }
        set_job(&env, &job);

        let token_client = token::Client::new(&env, &job.token);
        token_client.transfer(&env.current_contract_address(), &job.freelancer, &payout_amount);

        emit_milestone_approved(&env, job_id, milestone_index, payout_amount);
        if is_completed {
            emit_job_completed(&env, job_id);
        }
        Ok(())
    }

    /// Client requests a revision on a submitted milestone (capped at 2 revisions)
    pub fn request_revision(
        env: Env,
        job_id: u64,
        milestone_index: u32,
        feedback_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        let job = get_job(&env, job_id)?;
        job.client.require_auth();

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Submitted {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let now = env.ledger().timestamp();
        if now > milestone.submitted_at + job.terms.review_window_seconds {
            return Err(ContractError::ReviewWindowElapsed);
        }

        if milestone.revision_count >= 2 {
            return Err(ContractError::MaxRevisionsExceeded);
        }

        milestone.revision_count = milestone.revision_count.saturating_add(1);
        milestone.revision_feedback_hash = Some(feedback_hash.clone());
        milestone.status = MilestoneStatus::RevisionRequested;
        set_milestone(&env, job_id, milestone_index, &milestone);

        emit_revision_requested(&env, job_id, milestone_index, milestone.revision_count, feedback_hash);
        Ok(())
    }

    /// Auto-release milestone funds if client is silent past the review window
    pub fn auto_release(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Submitted {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let now = env.ledger().timestamp();
        if now <= milestone.submitted_at + job.terms.review_window_seconds {
            return Err(ContractError::ReviewWindowNotElapsed);
        }

        let payout_amount = milestone.amount;
        milestone.status = MilestoneStatus::Approved;
        set_milestone(&env, job_id, milestone_index, &milestone);

        job.escrow_balance = job
            .escrow_balance
            .checked_sub(payout_amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        job.current_milestone_index = job.current_milestone_index.saturating_add(1);

        let is_completed = job.current_milestone_index >= job.milestone_count;
        if is_completed {
            job.status = JobStatus::Completed;
            decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);
        }
        set_job(&env, &job);

        let token_client = token::Client::new(&env, &job.token);
        token_client.transfer(&env.current_contract_address(), &job.freelancer, &payout_amount);

        emit_auto_released(&env, job_id, milestone_index, payout_amount);
        if is_completed {
            emit_job_completed(&env, job_id);
        }
        Ok(())
    }

    /// Client reclaims escrow if freelancer misses deadline or stalls
    pub fn reclaim_stalled(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.client.require_auth();

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Pending
            && milestone.status != MilestoneStatus::RevisionRequested
        {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let now = env.ledger().timestamp();
        let timeout_reached = (now > milestone.deadline)
            || (milestone.submitted_at > 0 && now > milestone.submitted_at + job.terms.work_timeout_seconds);

        if !timeout_reached {
            return Err(ContractError::DeadlineNotPassed);
        }

        let refund_amount = job.escrow_balance;
        job.escrow_balance = 0;
        job.status = JobStatus::StalledReclaimed;
        set_job(&env, &job);

        decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);

        let token_client = token::Client::new(&env, &job.token);
        token_client.transfer(&env.current_contract_address(), &job.client, &refund_amount);

        emit_stalled_reclaimed(&env, job_id, &job.client, refund_amount);
        Ok(())
    }

    /// Freelancer voluntarily abandons job, returning remaining escrow to client
    pub fn abandon_job(env: Env, job_id: u64) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.freelancer.require_auth();

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let refund_amount = job.escrow_balance;
        job.escrow_balance = 0;
        job.status = JobStatus::Abandoned;
        set_job(&env, &job);

        decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);

        let token_client = token::Client::new(&env, &job.token);
        token_client.transfer(&env.current_contract_address(), &job.client, &refund_amount);

        emit_job_abandoned(&env, job_id, &job.freelancer, refund_amount);
        Ok(())
    }

    /// Bilateral mutual close with agreed partial payouts
    pub fn mutual_close(
        env: Env,
        job_id: u64,
        client_amount: i128,
        freelancer_amount: i128,
    ) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        job.client.require_auth();
        job.freelancer.require_auth();

        if job.status != JobStatus::Active && job.status != JobStatus::Disputed {
            return Err(ContractError::InvalidJobStatus);
        }

        if client_amount < 0 || freelancer_amount < 0 {
            return Err(ContractError::ZeroAmount);
        }

        let total_split = client_amount
            .checked_add(freelancer_amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if total_split != job.escrow_balance {
            return Err(ContractError::InvalidSplitTotal);
        }

        job.escrow_balance = 0;
        job.status = JobStatus::Closed;
        set_job(&env, &job);

        decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);

        let token_client = token::Client::new(&env, &job.token);
        if client_amount > 0 {
            token_client.transfer(&env.current_contract_address(), &job.client, &client_amount);
        }
        if freelancer_amount > 0 {
            token_client.transfer(&env.current_contract_address(), &job.freelancer, &freelancer_amount);
        }

        emit_job_closed(&env, job_id, client_amount, freelancer_amount);
        Ok(())
    }

    /// Open dispute on a milestone during review or after revision
    pub fn open_dispute(
        env: Env,
        job_id: u64,
        milestone_index: u32,
        caller: Address,
        reason_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        caller.require_auth();

        let mut job = get_job(&env, job_id)?;
        if caller != job.client && caller != job.freelancer {
            return Err(ContractError::Unauthorized);
        }

        if job.status != JobStatus::Active {
            return Err(ContractError::InvalidJobStatus);
        }

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        if milestone.status != MilestoneStatus::Submitted
            && milestone.status != MilestoneStatus::RevisionRequested
        {
            return Err(ContractError::InvalidMilestoneStatus);
        }

        let now = env.ledger().timestamp();
        // If submitted, must be within review window
        if milestone.status == MilestoneStatus::Submitted
            && now > milestone.submitted_at + job.terms.review_window_seconds
        {
            return Err(ContractError::ReviewWindowElapsed);
        }

        let fee_amount = (milestone.amount * (job.terms.arbitrator_fee_bps as i128)) / 10000;
        let dispute = Dispute {
            job_id,
            milestone_index,
            opened_by: caller.clone(),
            opened_at: now,
            voting_deadline: now + job.terms.dispute_voting_window_seconds,
            status: DisputeStatus::Active,
            reason_hash: reason_hash.clone(),
            total_disputed_amount: milestone.amount,
            arbitrator_fee_amount: fee_amount,
            settled_freelancer_share_bps: 0,
        };

        set_dispute(&env, job_id, milestone_index, &dispute);

        milestone.status = MilestoneStatus::Disputed;
        set_milestone(&env, job_id, milestone_index, &milestone);

        job.status = JobStatus::Disputed;
        set_job(&env, &job);

        emit_dispute_opened(&env, job_id, milestone_index, &caller, reason_hash);
        Ok(())
    }

    /// Submit cryptographic evidence hash for an open dispute
    pub fn submit_evidence(
        env: Env,
        job_id: u64,
        milestone_index: u32,
        submitter: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        submitter.require_auth();

        let job = get_job(&env, job_id)?;
        if submitter != job.client && submitter != job.freelancer {
            return Err(ContractError::Unauthorized);
        }

        let dispute = get_dispute(&env, job_id, milestone_index)?;
        if dispute.status != DisputeStatus::Active {
            return Err(ContractError::DisputeAlreadyResolved);
        }

        let record = EvidenceRecord {
            submitter: submitter.clone(),
            evidence_hash: evidence_hash.clone(),
            submitted_at: env.ledger().timestamp(),
        };
        add_evidence(&env, job_id, milestone_index, &record);

        emit_evidence_submitted(&env, job_id, milestone_index, &submitter, evidence_hash);
        Ok(())
    }

    /// Arbitrator casts vote on dispute proposing freelancer share percentage in basis points (0..=10000)
    pub fn vote_dispute(
        env: Env,
        job_id: u64,
        milestone_index: u32,
        arbitrator: Address,
        freelancer_share_bps: u32,
    ) -> Result<(), ContractError> {
        arbitrator.require_auth();

        if freelancer_share_bps > 10000 {
            return Err(ContractError::InvalidPercentage);
        }

        let job = get_job(&env, job_id)?;
        let dispute = get_dispute(&env, job_id, milestone_index)?;
        if dispute.status != DisputeStatus::Active {
            return Err(ContractError::DisputeAlreadyResolved);
        }

        let now = env.ledger().timestamp();
        if now > dispute.voting_deadline {
            return Err(ContractError::DeadlinePassed);
        }

        // Check arbitrator membership
        let mut is_on_panel = false;
        for i in 0..job.arbitrator_panel.len() {
            if job.arbitrator_panel.get(i).unwrap() == arbitrator {
                is_on_panel = true;
                break;
            }
        }
        if !is_on_panel {
            return Err(ContractError::ArbitratorNotOnPanel);
        }

        if get_vote(&env, job_id, milestone_index, &arbitrator).is_some() {
            return Err(ContractError::ArbitratorAlreadyVoted);
        }

        let vote = ArbitratorVote {
            arbitrator: arbitrator.clone(),
            freelancer_share_bps,
            voted_at: now,
        };
        set_vote(&env, job_id, milestone_index, &vote);

        let mut voter_list = get_voter_list(&env, job_id, milestone_index);
        voter_list.push_back(arbitrator.clone());
        set_voter_list(&env, job_id, milestone_index, &voter_list);

        let mut stats = get_arbitrator_stats(&env, &arbitrator);
        stats.total_cases_voted = stats.total_cases_voted.saturating_add(1);
        set_arbitrator_stats(&env, &arbitrator, &stats);

        emit_dispute_voted(&env, job_id, milestone_index, &arbitrator, freelancer_share_bps);
        Ok(())
    }

    /// Resolve dispute via majority consensus or voting deadline settlement
    pub fn resolve_dispute(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<(), ContractError> {
        let mut job = get_job(&env, job_id)?;
        let mut dispute = get_dispute(&env, job_id, milestone_index)?;
        if dispute.status != DisputeStatus::Active {
            return Err(ContractError::DisputeAlreadyResolved);
        }

        let now = env.ledger().timestamp();
        let voters = get_voter_list(&env, job_id, milestone_index);
        let panel_size = job.arbitrator_panel.len();
        let majority_threshold = (panel_size / 2) + 1;

        let deadline_passed = now > dispute.voting_deadline;
        let all_voted = voters.len() == panel_size;

        // Check if there is an exact majority outcome early
        let mut early_majority_outcome: Option<u32> = None;
        if voters.len() >= majority_threshold {
            // Count frequencies
            for i in 0..voters.len() {
                let arb_i = voters.get(i).unwrap();
                let vote_i = get_vote(&env, job_id, milestone_index, &arb_i).unwrap();
                let mut count = 0;
                for j in 0..voters.len() {
                    let arb_j = voters.get(j).unwrap();
                    let vote_j = get_vote(&env, job_id, milestone_index, &arb_j).unwrap();
                    if vote_i.freelancer_share_bps == vote_j.freelancer_share_bps {
                        count += 1;
                    }
                }
                if count >= majority_threshold {
                    early_majority_outcome = Some(vote_i.freelancer_share_bps);
                    break;
                }
            }
        }

        if !deadline_passed && !all_voted && early_majority_outcome.is_none() {
            return Err(ContractError::DisputeNotResolvableYet);
        }

        // Determine final outcome
        let settled_freelancer_share_bps: u32 = if let Some(majority_bps) = early_majority_outcome {
            majority_bps
        } else if voters.len() == 0 {
            5000 // 50/50 split on zero votes
        } else {
            // If deadline passed: choose most voted, or average if no single winner, or 5000 on tie
            let mut best_count = 0;
            let mut best_bps = 5000;
            let mut is_tie = false;

            for i in 0..voters.len() {
                let arb_i = voters.get(i).unwrap();
                let vote_i = get_vote(&env, job_id, milestone_index, &arb_i).unwrap();
                let mut count = 0;
                for j in 0..voters.len() {
                    let arb_j = voters.get(j).unwrap();
                    let vote_j = get_vote(&env, job_id, milestone_index, &arb_j).unwrap();
                    if vote_i.freelancer_share_bps == vote_j.freelancer_share_bps {
                        count += 1;
                    }
                }
                if count > best_count {
                    best_count = count;
                    best_bps = vote_i.freelancer_share_bps;
                    is_tie = false;
                } else if count == best_count && best_bps != vote_i.freelancer_share_bps {
                    is_tie = true;
                }
            }

            if is_tie {
                5000
            } else {
                best_bps
            }
        };

        // Fee distribution among participating voters
        let total_disputed = dispute.total_disputed_amount;
        let mut actual_arb_fee: i128 = 0;
        let token_client = token::Client::new(&env, &job.token);

        if voters.len() > 0 && dispute.arbitrator_fee_amount > 0 {
            actual_arb_fee = dispute.arbitrator_fee_amount;
            let fee_per_voter = actual_arb_fee / (voters.len() as i128);
            let remainder = actual_arb_fee - (fee_per_voter * (voters.len() as i128));

            for i in 0..voters.len() {
                let arb = voters.get(i).unwrap();
                let payout = if i == 0 {
                    fee_per_voter + remainder
                } else {
                    fee_per_voter
                };
                if payout > 0 {
                    token_client.transfer(&env.current_contract_address(), &arb, &payout);
                    let mut stats = get_arbitrator_stats(&env, &arb);
                    stats.total_fees_earned = stats.total_fees_earned.saturating_add(payout);
                    set_arbitrator_stats(&env, &arb, &stats);
                }
            }
        }

        // Slashing non-voting arbitrators if deadline passed and they did not participate
        if deadline_passed && voters.len() < panel_size {
            for i in 0..panel_size {
                let arb = job.arbitrator_panel.get(i).unwrap();
                let mut has_voted = false;
                for v in 0..voters.len() {
                    if voters.get(v).unwrap() == arb {
                        has_voted = true;
                        break;
                    }
                }
                if !has_voted {
                    // Slash 10% of required stake or a penalty
                    let mut stake = get_arbitrator_stake(&env, &arb, &job.token);
                    let slash_amount = if job.terms.arbitrator_stake_required > 0 {
                        job.terms.arbitrator_stake_required / 10
                    } else {
                        0
                    };
                    if slash_amount > 0 && stake.staked_amount >= slash_amount {
                        stake.staked_amount -= slash_amount;
                        stake.slashed_amount += slash_amount;
                        set_arbitrator_stake(&env, &arb, &job.token, &stake);

                        let mut stats = get_arbitrator_stats(&env, &arb);
                        stats.total_cases_slashed = stats.total_cases_slashed.saturating_add(1);
                        set_arbitrator_stats(&env, &arb, &stats);

                        emit_arbitrator_slashed(&env, &arb, &job.token, slash_amount, SlashingReason::NonVoting as u32);
                    }
                }
            }
        }

        // Remaining amount for client and freelancer
        let principal_remaining = total_disputed - actual_arb_fee;
        let freelancer_payout = (principal_remaining * (settled_freelancer_share_bps as i128)) / 10000;
        let client_payout = principal_remaining - freelancer_payout;

        // Checks-Effects-Interactions: update state before transfers
        dispute.status = DisputeStatus::Resolved;
        dispute.settled_freelancer_share_bps = settled_freelancer_share_bps;
        set_dispute(&env, job_id, milestone_index, &dispute);

        let mut milestone = get_milestone(&env, job_id, milestone_index)?;
        milestone.status = MilestoneStatus::Resolved;
        set_milestone(&env, job_id, milestone_index, &milestone);

        job.escrow_balance = job
            .escrow_balance
            .checked_sub(total_disputed)
            .ok_or(ContractError::ArithmeticOverflow)?;
        job.current_milestone_index = job.current_milestone_index.saturating_add(1);

        let is_completed = job.current_milestone_index >= job.milestone_count;
        if is_completed {
            job.status = JobStatus::Completed;
            decrement_panel_active_count(&env, &job.arbitrator_panel, &job.token);
        } else {
            job.status = JobStatus::Active;
        }
        set_job(&env, &job);

        // Perform client and freelancer payouts
        if freelancer_payout > 0 {
            token_client.transfer(&env.current_contract_address(), &job.freelancer, &freelancer_payout);
        }
        if client_payout > 0 {
            token_client.transfer(&env.current_contract_address(), &job.client, &client_payout);
        }

        emit_dispute_resolved(&env, job_id, milestone_index, settled_freelancer_share_bps, actual_arb_fee);
        if is_completed {
            emit_job_completed(&env, job_id);
        }
        Ok(())
    }

    /// Admin slash for provably malicious or biased arbitrator
    pub fn slash_arbitrator(
        env: Env,
        admin: Address,
        arbitrator: Address,
        token: Address,
        amount: i128,
        reason: u32,
    ) -> Result<(), ContractError> {
        admin.require_auth();
        let current_admin = get_admin(&env)?;
        if admin != current_admin {
            return Err(ContractError::Unauthorized);
        }

        let mut stake = get_arbitrator_stake(&env, &arbitrator, &token);
        if stake.staked_amount < amount {
            return Err(ContractError::ArbitratorInsufficientStake);
        }

        stake.staked_amount = stake
            .staked_amount
            .checked_sub(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        stake.slashed_amount = stake
            .slashed_amount
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        set_arbitrator_stake(&env, &arbitrator, &token, &stake);

        let mut stats = get_arbitrator_stats(&env, &arbitrator);
        stats.total_cases_slashed = stats.total_cases_slashed.saturating_add(1);
        set_arbitrator_stats(&env, &arbitrator, &stats);

        // Send slashed funds to admin / treasury
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &admin, &amount);

        emit_arbitrator_slashed(&env, &arbitrator, &token, amount, reason);
        Ok(())
    }

    // --- GETTERS ---

    pub fn get_job(env: Env, job_id: u64) -> Result<Job, ContractError> {
        get_job(&env, job_id)
    }

    pub fn get_milestone(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<Milestone, ContractError> {
        get_milestone(&env, job_id, milestone_index)
    }

    pub fn get_milestones(env: Env, job_id: u64) -> Result<Vec<Milestone>, ContractError> {
        let job = get_job(&env, job_id)?;
        let mut list = Vec::new(&env);
        for i in 0..job.milestone_count {
            list.push_back(get_milestone(&env, job_id, i)?);
        }
        Ok(list)
    }

    pub fn get_dispute(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Result<Dispute, ContractError> {
        get_dispute(&env, job_id, milestone_index)
    }

    pub fn get_votes(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Vec<ArbitratorVote> {
        let voters = get_voter_list(&env, job_id, milestone_index);
        let mut list = Vec::new(&env);
        for i in 0..voters.len() {
            let arb = voters.get(i).unwrap();
            if let Some(v) = get_vote(&env, job_id, milestone_index, &arb) {
                list.push_back(v);
            }
        }
        list
    }

    pub fn get_evidence(
        env: Env,
        job_id: u64,
        milestone_index: u32,
    ) -> Vec<EvidenceRecord> {
        get_evidence_list(&env, job_id, milestone_index)
    }

    pub fn get_arbitrator_stake(
        env: Env,
        arbitrator: Address,
        token: Address,
    ) -> ArbitratorStake {
        get_arbitrator_stake(&env, &arbitrator, &token)
    }

    pub fn get_arbitrator_stats(env: Env, arbitrator: Address) -> ArbitratorStats {
        get_arbitrator_stats(&env, &arbitrator)
    }

    pub fn get_job_count(env: Env) -> u64 {
        get_job_count(&env)
    }
}

fn decrement_panel_active_count(env: &Env, panel: &Vec<Address>, token: &Address) {
    for i in 0..panel.len() {
        let arb = panel.get(i).unwrap();
        let mut stake = get_arbitrator_stake(env, &arb, token);
        stake.active_panels_count = stake.active_panels_count.saturating_sub(1);
        set_arbitrator_stake(env, &arb, token, &stake);
    }
}
