use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    JobNotFound = 4,
    MilestoneNotFound = 5,
    InvalidJobStatus = 6,
    InvalidMilestoneStatus = 7,
    InvalidMilestoneCount = 8,
    InvalidMilestoneAmount = 9,
    InvalidMilestoneDeadline = 10,
    InvalidArbitratorPanel = 11,
    ArbitratorAlreadyVoted = 12,
    ArbitratorNotOnPanel = 13,
    ArbitratorInsufficientStake = 14,
    MaxRevisionsExceeded = 15,
    ReviewWindowNotElapsed = 16,
    ReviewWindowElapsed = 17,
    DeadlinePassed = 18,
    DeadlineNotPassed = 19,
    DisputeAlreadyOpen = 20,
    DisputeNotFound = 21,
    DisputeAlreadyResolved = 22,
    DisputeNotResolvableYet = 23,
    InvalidPercentage = 24,
    InvalidSplitTotal = 25,
    InsufficientEscrow = 26,
    ArithmeticOverflow = 27,
    ZeroAmount = 28,
    ActiveCasesPending = 29,
    CannotSelfArbitrate = 30,
}
