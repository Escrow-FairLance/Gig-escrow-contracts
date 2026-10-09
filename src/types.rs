use soroban_sdk::{contracttype, Address, BytesN, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobStatus {
    Created = 0,
    Active = 1,
    Disputed = 2,
    Completed = 3,
    Cancelled = 4,
    Declined = 5,
    StalledReclaimed = 6,
    Abandoned = 7,
    Closed = 8,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MilestoneStatus {
    Pending = 0,
    Submitted = 1,
    RevisionRequested = 2,
    Approved = 3,
    Disputed = 4,
    Resolved = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    Active = 0,
    Resolved = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JobTerms {
    pub review_window_seconds: u64,
    pub work_timeout_seconds: u64,
    pub dispute_voting_window_seconds: u64,
    pub arbitrator_fee_bps: u32,
    pub arbitrator_stake_required: i128,
    pub client_cancel_fee_bps: u32,
    pub template_hash: BytesN<32>,
    pub terms_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneInit {
    pub title_hash: BytesN<32>,
    pub amount: i128,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub title_hash: BytesN<32>,
    pub amount: i128,
    pub deadline: u64,
    pub status: MilestoneStatus,
    pub deliverable_hash: Option<BytesN<32>>,
    pub submitted_at: u64,
    pub revision_count: u32,
    pub revision_feedback_hash: Option<BytesN<32>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub job_id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub token: Address,
    pub terms: JobTerms,
    pub total_amount: i128,
    pub escrow_balance: i128,
    pub arbitrator_panel: Vec<Address>,
    pub status: JobStatus,
    pub milestone_count: u32,
    pub current_milestone_index: u32,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitratorVote {
    pub arbitrator: Address,
    pub freelancer_share_bps: u32,
    pub voted_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub job_id: u64,
    pub milestone_index: u32,
    pub opened_by: Address,
    pub opened_at: u64,
    pub voting_deadline: u64,
    pub status: DisputeStatus,
    pub reason_hash: BytesN<32>,
    pub total_disputed_amount: i128,
    pub arbitrator_fee_amount: i128,
    pub settled_freelancer_share_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRecord {
    pub submitter: Address,
    pub evidence_hash: BytesN<32>,
    pub submitted_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitratorStake {
    pub staked_amount: i128,
    pub slashed_amount: i128,
    pub active_panels_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArbitratorStats {
    pub total_cases_assigned: u32,
    pub total_cases_voted: u32,
    pub total_cases_slashed: u32,
    pub total_fees_earned: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SlashingReason {
    NonVoting = 0,
    BiasedOutlier = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    JobCount,
    Job(u64),
    Milestone(u64, u32),
    Dispute(u64, u32),
    Vote(u64, u32, Address),
    VoterList(u64, u32),
    EvidenceList(u64, u32),
    ArbitratorStake(Address, Address), // (arbitrator, token)
    ArbitratorStats(Address),
}
