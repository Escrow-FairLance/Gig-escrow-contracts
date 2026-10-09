import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CDG3CB5IEATXZPTL3SQGCBVJM2A4HUX4J5TTFGMA7NPGFQBMQBYFXZCB",
  }
} as const


export interface Job {
  arbitrator_panel: Array<string>;
  client: string;
  created_at: u64;
  current_milestone_index: u32;
  escrow_balance: i128;
  freelancer: string;
  job_id: u64;
  milestone_count: u32;
  status: JobStatus;
  terms: JobTerms;
  token: string;
  total_amount: i128;
}

export type DataKey = {tag: "Admin", values: void} | {tag: "JobCount", values: void} | {tag: "Job", values: readonly [u64]} | {tag: "Milestone", values: readonly [u64, u32]} | {tag: "Dispute", values: readonly [u64, u32]} | {tag: "Vote", values: readonly [u64, u32, string]} | {tag: "VoterList", values: readonly [u64, u32]} | {tag: "EvidenceList", values: readonly [u64, u32]} | {tag: "ArbitratorStake", values: readonly [string, string]} | {tag: "ArbitratorStats", values: readonly [string]};


export interface Dispute {
  arbitrator_fee_amount: i128;
  job_id: u64;
  milestone_index: u32;
  opened_at: u64;
  opened_by: string;
  reason_hash: Buffer;
  settled_freelancer_share_bps: u32;
  status: DisputeStatus;
  total_disputed_amount: i128;
  voting_deadline: u64;
}


export interface JobTerms {
  arbitrator_fee_bps: u32;
  arbitrator_stake_required: i128;
  client_cancel_fee_bps: u32;
  dispute_voting_window_seconds: u64;
  review_window_seconds: u64;
  template_hash: Buffer;
  terms_hash: Buffer;
  work_timeout_seconds: u64;
}

export enum JobStatus {
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


export interface Milestone {
  amount: i128;
  deadline: u64;
  deliverable_hash: Option<Buffer>;
  revision_count: u32;
  revision_feedback_hash: Option<Buffer>;
  status: MilestoneStatus;
  submitted_at: u64;
  title_hash: Buffer;
}

export enum DisputeStatus {
  Active = 0,
  Resolved = 1,
}


export interface MilestoneInit {
  amount: i128;
  deadline: u64;
  title_hash: Buffer;
}


export interface ArbitratorVote {
  arbitrator: string;
  freelancer_share_bps: u32;
  voted_at: u64;
}


export interface EvidenceRecord {
  evidence_hash: Buffer;
  submitted_at: u64;
  submitter: string;
}

export enum SlashingReason {
  NonVoting = 0,
  BiasedOutlier = 1,
}


export interface ArbitratorStake {
  active_panels_count: u32;
  slashed_amount: i128;
  staked_amount: i128;
}


export interface ArbitratorStats {
  total_cases_assigned: u32;
  total_cases_slashed: u32;
  total_cases_voted: u32;
  total_fees_earned: i128;
}

export enum MilestoneStatus {
  Pending = 0,
  Submitted = 1,
  RevisionRequested = 2,
  Approved = 3,
  Disputed = 4,
  Resolved = 5,
}

export const Errors = {
  1: {message:"NotInitialized"},

  2: {message:"AlreadyInitialized"},

  3: {message:"Unauthorized"},

  4: {message:"JobNotFound"},

  5: {message:"MilestoneNotFound"},

  6: {message:"InvalidJobStatus"},

  7: {message:"InvalidMilestoneStatus"},

  8: {message:"InvalidMilestoneCount"},

  9: {message:"InvalidMilestoneAmount"},

  10: {message:"InvalidMilestoneDeadline"},

  11: {message:"InvalidArbitratorPanel"},

  12: {message:"ArbitratorAlreadyVoted"},

  13: {message:"ArbitratorNotOnPanel"},

  14: {message:"ArbitratorInsufficientStake"},

  15: {message:"MaxRevisionsExceeded"},

  16: {message:"ReviewWindowNotElapsed"},

  17: {message:"ReviewWindowElapsed"},

  18: {message:"DeadlinePassed"},

  19: {message:"DeadlineNotPassed"},

  20: {message:"DisputeAlreadyOpen"},

  21: {message:"DisputeNotFound"},

  22: {message:"DisputeAlreadyResolved"},

  23: {message:"DisputeNotResolvableYet"},

  24: {message:"InvalidPercentage"},

  25: {message:"InvalidSplitTotal"},

  26: {message:"InsufficientEscrow"},

  27: {message:"ArithmeticOverflow"},

  28: {message:"ZeroAmount"},

  29: {message:"ActiveCasesPending"},

  30: {message:"CannotSelfArbitrate"}
}

export interface Client {
  /**
   * Construct and simulate a get_job transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_job: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Job>>>

  /**
   * Construct and simulate a get_votes transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_votes: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<ArbitratorVote>>>

  /**
   * Construct and simulate a accept_job transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Freelancer accepts the job offer
   */
  accept_job: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a create_job transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new escrow job, funding the full milestone amount
   */
  create_job: ({client, freelancer, token, milestones, terms, arbitrator_panel}: {client: string, freelancer: string, token: string, milestones: Array<MilestoneInit>, terms: JobTerms, arbitrator_panel: Array<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the contract with an admin address
   */
  initialize: ({admin}: {admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a abandon_job transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Freelancer voluntarily abandons job, returning remaining escrow to client
   */
  abandon_job: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a decline_job transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Freelancer declines the job offer, issuing a 100% refund to the client
   */
  decline_job: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_dispute: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Dispute>>>

  /**
   * Construct and simulate a auto_release transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Auto-release milestone funds if client is silent past the review window
   */
  auto_release: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_evidence transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_evidence: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<EvidenceRecord>>>

  /**
   * Construct and simulate a mutual_close transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Bilateral mutual close with agreed partial payouts
   */
  mutual_close: ({job_id, client_amount, freelancer_amount}: {job_id: u64, client_amount: i128, freelancer_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a open_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Open dispute on a milestone during review or after revision
   */
  open_dispute: ({job_id, milestone_index, caller, reason_hash}: {job_id: u64, milestone_index: u32, caller: string, reason_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a vote_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Arbitrator casts vote on dispute proposing freelancer share percentage in basis points (0..=10000)
   */
  vote_dispute: ({job_id, milestone_index, arbitrator, freelancer_share_bps}: {job_id: u64, milestone_index: u32, arbitrator: string, freelancer_share_bps: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_job_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_job_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_milestone: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Milestone>>>

  /**
   * Construct and simulate a get_milestones transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_milestones: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Array<Milestone>>>>

  /**
   * Construct and simulate a reclaim_stalled transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Client reclaims escrow if freelancer misses deadline or stalls
   */
  reclaim_stalled: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a resolve_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Resolve dispute via majority consensus or voting deadline settlement
   */
  resolve_dispute: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a submit_evidence transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Submit cryptographic evidence hash for an open dispute
   */
  submit_evidence: ({job_id, milestone_index, submitter, evidence_hash}: {job_id: u64, milestone_index: u32, submitter: string, evidence_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a request_revision transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Client requests a revision on a submitted milestone (capped at 2 revisions)
   */
  request_revision: ({job_id, milestone_index, feedback_hash}: {job_id: u64, milestone_index: u32, feedback_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a slash_arbitrator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Admin slash for provably malicious or biased arbitrator
   */
  slash_arbitrator: ({admin, arbitrator, token, amount, reason}: {admin: string, arbitrator: string, token: string, amount: i128, reason: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a stake_arbitrator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Stake tokens as an arbitrator to be eligible for selection in panels
   */
  stake_arbitrator: ({arbitrator, token, amount}: {arbitrator: string, token: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a submit_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Freelancer submits milestone deliverables (hash of files)
   */
  submit_milestone: ({job_id, milestone_index, deliverable_hash}: {job_id: u64, milestone_index: u32, deliverable_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a approve_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Client approves milestone deliverable, triggering instant payout
   */
  approve_milestone: ({job_id, milestone_index}: {job_id: u64, milestone_index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a cancel_unaccepted transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Client cancels job before freelancer acceptance (applies cancellation fee if set)
   */
  cancel_unaccepted: ({job_id}: {job_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a unstake_arbitrator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Withdraw staked arbitrator collateral if no active cases or surplus exists
   */
  unstake_arbitrator: ({arbitrator, token, amount}: {arbitrator: string, token: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_arbitrator_stake transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_arbitrator_stake: ({arbitrator, token}: {arbitrator: string, token: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ArbitratorStake>>

  /**
   * Construct and simulate a get_arbitrator_stats transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_arbitrator_stats: ({arbitrator}: {arbitrator: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ArbitratorStats>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAQAAAAAAAAAAAAAAA0pvYgAAAAAMAAAAAAAAABBhcmJpdHJhdG9yX3BhbmVsAAAD6gAAABMAAAAAAAAABmNsaWVudAAAAAAAEwAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAAXY3VycmVudF9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAAAAAAOZXNjcm93X2JhbGFuY2UAAAAAAAsAAAAAAAAACmZyZWVsYW5jZXIAAAAAABMAAAAAAAAABmpvYl9pZAAAAAAABgAAAAAAAAAPbWlsZXN0b25lX2NvdW50AAAAAAQAAAAAAAAABnN0YXR1cwAAAAAH0AAAAAlKb2JTdGF0dXMAAAAAAAAAAAAABXRlcm1zAAAAAAAH0AAAAAhKb2JUZXJtcwAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAAAAAAx0b3RhbF9hbW91bnQAAAAL",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAACgAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAISm9iQ291bnQAAAABAAAAAAAAAANKb2IAAAAAAQAAAAYAAAABAAAAAAAAAAlNaWxlc3RvbmUAAAAAAAACAAAABgAAAAQAAAABAAAAAAAAAAdEaXNwdXRlAAAAAAIAAAAGAAAABAAAAAEAAAAAAAAABFZvdGUAAAADAAAABgAAAAQAAAATAAAAAQAAAAAAAAAJVm90ZXJMaXN0AAAAAAAAAgAAAAYAAAAEAAAAAQAAAAAAAAAMRXZpZGVuY2VMaXN0AAAAAgAAAAYAAAAEAAAAAQAAAAAAAAAPQXJiaXRyYXRvclN0YWtlAAAAAAIAAAATAAAAEwAAAAEAAAAAAAAAD0FyYml0cmF0b3JTdGF0cwAAAAABAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAB0Rpc3B1dGUAAAAACgAAAAAAAAAVYXJiaXRyYXRvcl9mZWVfYW1vdW50AAAAAAAACwAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAAAAAAJb3BlbmVkX2F0AAAAAAAABgAAAAAAAAAJb3BlbmVkX2J5AAAAAAAAEwAAAAAAAAALcmVhc29uX2hhc2gAAAAD7gAAACAAAAAAAAAAHHNldHRsZWRfZnJlZWxhbmNlcl9zaGFyZV9icHMAAAAEAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAANRGlzcHV0ZVN0YXR1cwAAAAAAAAAAAAAVdG90YWxfZGlzcHV0ZWRfYW1vdW50AAAAAAAACwAAAAAAAAAPdm90aW5nX2RlYWRsaW5lAAAAAAY=",
        "AAAAAQAAAAAAAAAAAAAACEpvYlRlcm1zAAAACAAAAAAAAAASYXJiaXRyYXRvcl9mZWVfYnBzAAAAAAAEAAAAAAAAABlhcmJpdHJhdG9yX3N0YWtlX3JlcXVpcmVkAAAAAAAACwAAAAAAAAAVY2xpZW50X2NhbmNlbF9mZWVfYnBzAAAAAAAABAAAAAAAAAAdZGlzcHV0ZV92b3Rpbmdfd2luZG93X3NlY29uZHMAAAAAAAAGAAAAAAAAABVyZXZpZXdfd2luZG93X3NlY29uZHMAAAAAAAAGAAAAAAAAAA10ZW1wbGF0ZV9oYXNoAAAAAAAD7gAAACAAAAAAAAAACnRlcm1zX2hhc2gAAAAAA+4AAAAgAAAAAAAAABR3b3JrX3RpbWVvdXRfc2Vjb25kcwAAAAY=",
        "AAAAAwAAAAAAAAAAAAAACUpvYlN0YXR1cwAAAAAAAAkAAAAAAAAAB0NyZWF0ZWQAAAAAAAAAAAAAAAAGQWN0aXZlAAAAAAABAAAAAAAAAAhEaXNwdXRlZAAAAAIAAAAAAAAACUNvbXBsZXRlZAAAAAAAAAMAAAAAAAAACUNhbmNlbGxlZAAAAAAAAAQAAAAAAAAACERlY2xpbmVkAAAABQAAAAAAAAAQU3RhbGxlZFJlY2xhaW1lZAAAAAYAAAAAAAAACUFiYW5kb25lZAAAAAAAAAcAAAAAAAAABkNsb3NlZAAAAAAACA==",
        "AAAAAQAAAAAAAAAAAAAACU1pbGVzdG9uZQAAAAAAAAgAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAIZGVhZGxpbmUAAAAGAAAAAAAAABBkZWxpdmVyYWJsZV9oYXNoAAAD6AAAA+4AAAAgAAAAAAAAAA5yZXZpc2lvbl9jb3VudAAAAAAABAAAAAAAAAAWcmV2aXNpb25fZmVlZGJhY2tfaGFzaAAAAAAD6AAAA+4AAAAgAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAAPTWlsZXN0b25lU3RhdHVzAAAAAAAAAAAMc3VibWl0dGVkX2F0AAAABgAAAAAAAAAKdGl0bGVfaGFzaAAAAAAD7gAAACA=",
        "AAAAAwAAAAAAAAAAAAAADURpc3B1dGVTdGF0dXMAAAAAAAACAAAAAAAAAAZBY3RpdmUAAAAAAAAAAAAAAAAACFJlc29sdmVkAAAAAQ==",
        "AAAAAQAAAAAAAAAAAAAADU1pbGVzdG9uZUluaXQAAAAAAAADAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAACGRlYWRsaW5lAAAABgAAAAAAAAAKdGl0bGVfaGFzaAAAAAAD7gAAACA=",
        "AAAAAQAAAAAAAAAAAAAADkFyYml0cmF0b3JWb3RlAAAAAAADAAAAAAAAAAphcmJpdHJhdG9yAAAAAAATAAAAAAAAABRmcmVlbGFuY2VyX3NoYXJlX2JwcwAAAAQAAAAAAAAACHZvdGVkX2F0AAAABg==",
        "AAAAAQAAAAAAAAAAAAAADkV2aWRlbmNlUmVjb3JkAAAAAAADAAAAAAAAAA1ldmlkZW5jZV9oYXNoAAAAAAAD7gAAACAAAAAAAAAADHN1Ym1pdHRlZF9hdAAAAAYAAAAAAAAACXN1Ym1pdHRlcgAAAAAAABM=",
        "AAAAAwAAAAAAAAAAAAAADlNsYXNoaW5nUmVhc29uAAAAAAACAAAAAAAAAAlOb25Wb3RpbmcAAAAAAAAAAAAAAAAAAA1CaWFzZWRPdXRsaWVyAAAAAAAAAQ==",
        "AAAAAQAAAAAAAAAAAAAAD0FyYml0cmF0b3JTdGFrZQAAAAADAAAAAAAAABNhY3RpdmVfcGFuZWxzX2NvdW50AAAAAAQAAAAAAAAADnNsYXNoZWRfYW1vdW50AAAAAAALAAAAAAAAAA1zdGFrZWRfYW1vdW50AAAAAAAACw==",
        "AAAAAQAAAAAAAAAAAAAAD0FyYml0cmF0b3JTdGF0cwAAAAAEAAAAAAAAABR0b3RhbF9jYXNlc19hc3NpZ25lZAAAAAQAAAAAAAAAE3RvdGFsX2Nhc2VzX3NsYXNoZWQAAAAABAAAAAAAAAARdG90YWxfY2FzZXNfdm90ZWQAAAAAAAAEAAAAAAAAABF0b3RhbF9mZWVzX2Vhcm5lZAAAAAAAAAs=",
        "AAAAAwAAAAAAAAAAAAAAD01pbGVzdG9uZVN0YXR1cwAAAAAGAAAAAAAAAAdQZW5kaW5nAAAAAAAAAAAAAAAACVN1Ym1pdHRlZAAAAAAAAAEAAAAAAAAAEVJldmlzaW9uUmVxdWVzdGVkAAAAAAAAAgAAAAAAAAAIQXBwcm92ZWQAAAADAAAAAAAAAAhEaXNwdXRlZAAAAAQAAAAAAAAACFJlc29sdmVkAAAABQ==",
        "AAAABAAAAAAAAAAAAAAADUNvbnRyYWN0RXJyb3IAAAAAAAAeAAAAAAAAAA5Ob3RJbml0aWFsaXplZAAAAAAAAQAAAAAAAAASQWxyZWFkeUluaXRpYWxpemVkAAAAAAACAAAAAAAAAAxVbmF1dGhvcml6ZWQAAAADAAAAAAAAAAtKb2JOb3RGb3VuZAAAAAAEAAAAAAAAABFNaWxlc3RvbmVOb3RGb3VuZAAAAAAAAAUAAAAAAAAAEEludmFsaWRKb2JTdGF0dXMAAAAGAAAAAAAAABZJbnZhbGlkTWlsZXN0b25lU3RhdHVzAAAAAAAHAAAAAAAAABVJbnZhbGlkTWlsZXN0b25lQ291bnQAAAAAAAAIAAAAAAAAABZJbnZhbGlkTWlsZXN0b25lQW1vdW50AAAAAAAJAAAAAAAAABhJbnZhbGlkTWlsZXN0b25lRGVhZGxpbmUAAAAKAAAAAAAAABZJbnZhbGlkQXJiaXRyYXRvclBhbmVsAAAAAAALAAAAAAAAABZBcmJpdHJhdG9yQWxyZWFkeVZvdGVkAAAAAAAMAAAAAAAAABRBcmJpdHJhdG9yTm90T25QYW5lbAAAAA0AAAAAAAAAG0FyYml0cmF0b3JJbnN1ZmZpY2llbnRTdGFrZQAAAAAOAAAAAAAAABRNYXhSZXZpc2lvbnNFeGNlZWRlZAAAAA8AAAAAAAAAFlJldmlld1dpbmRvd05vdEVsYXBzZWQAAAAAABAAAAAAAAAAE1Jldmlld1dpbmRvd0VsYXBzZWQAAAAAEQAAAAAAAAAORGVhZGxpbmVQYXNzZWQAAAAAABIAAAAAAAAAEURlYWRsaW5lTm90UGFzc2VkAAAAAAAAEwAAAAAAAAASRGlzcHV0ZUFscmVhZHlPcGVuAAAAAAAUAAAAAAAAAA9EaXNwdXRlTm90Rm91bmQAAAAAFQAAAAAAAAAWRGlzcHV0ZUFscmVhZHlSZXNvbHZlZAAAAAAAFgAAAAAAAAAXRGlzcHV0ZU5vdFJlc29sdmFibGVZZXQAAAAAFwAAAAAAAAARSW52YWxpZFBlcmNlbnRhZ2UAAAAAAAAYAAAAAAAAABFJbnZhbGlkU3BsaXRUb3RhbAAAAAAAABkAAAAAAAAAEkluc3VmZmljaWVudEVzY3JvdwAAAAAAGgAAAAAAAAASQXJpdGhtZXRpY092ZXJmbG93AAAAAAAbAAAAAAAAAApaZXJvQW1vdW50AAAAAAAcAAAAAAAAABJBY3RpdmVDYXNlc1BlbmRpbmcAAAAAAB0AAAAAAAAAE0Nhbm5vdFNlbGZBcmJpdHJhdGUAAAAAHg==",
        "AAAAAAAAAAAAAAAHZ2V0X2pvYgAAAAABAAAAAAAAAAZqb2JfaWQAAAAAAAYAAAABAAAD6QAAB9AAAAADSm9iAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAAAAAAAJZ2V0X3ZvdGVzAAAAAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPqAAAH0AAAAA5BcmJpdHJhdG9yVm90ZQAA",
        "AAAAAAAAACBGcmVlbGFuY2VyIGFjY2VwdHMgdGhlIGpvYiBvZmZlcgAAAAphY2NlcHRfam9iAAAAAAABAAAAAAAAAAZqb2JfaWQAAAAAAAYAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAADpDcmVhdGUgYSBuZXcgZXNjcm93IGpvYiwgZnVuZGluZyB0aGUgZnVsbCBtaWxlc3RvbmUgYW1vdW50AAAAAAAKY3JlYXRlX2pvYgAAAAAABgAAAAAAAAAGY2xpZW50AAAAAAATAAAAAAAAAApmcmVlbGFuY2VyAAAAAAATAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAAAAAACm1pbGVzdG9uZXMAAAAAA+oAAAfQAAAADU1pbGVzdG9uZUluaXQAAAAAAAAAAAAABXRlcm1zAAAAAAAH0AAAAAhKb2JUZXJtcwAAAAAAAAAQYXJiaXRyYXRvcl9wYW5lbAAAA+oAAAATAAAAAQAAA+kAAAAGAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAC1Jbml0aWFsaXplIHRoZSBjb250cmFjdCB3aXRoIGFuIGFkbWluIGFkZHJlc3MAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAElGcmVlbGFuY2VyIHZvbHVudGFyaWx5IGFiYW5kb25zIGpvYiwgcmV0dXJuaW5nIHJlbWFpbmluZyBlc2Nyb3cgdG8gY2xpZW50AAAAAAAAC2FiYW5kb25fam9iAAAAAAEAAAAAAAAABmpvYl9pZAAAAAAABgAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAEZGcmVlbGFuY2VyIGRlY2xpbmVzIHRoZSBqb2Igb2ZmZXIsIGlzc3VpbmcgYSAxMDAlIHJlZnVuZCB0byB0aGUgY2xpZW50AAAAAAALZGVjbGluZV9qb2IAAAAAAQAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAAAAAAALZ2V0X2Rpc3B1dGUAAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPpAAAH0AAAAAdEaXNwdXRlAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEdBdXRvLXJlbGVhc2UgbWlsZXN0b25lIGZ1bmRzIGlmIGNsaWVudCBpcyBzaWxlbnQgcGFzdCB0aGUgcmV2aWV3IHdpbmRvdwAAAAAMYXV0b19yZWxlYXNlAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAAAAAAAMZ2V0X2V2aWRlbmNlAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPqAAAH0AAAAA5FdmlkZW5jZVJlY29yZAAA",
        "AAAAAAAAADJCaWxhdGVyYWwgbXV0dWFsIGNsb3NlIHdpdGggYWdyZWVkIHBhcnRpYWwgcGF5b3V0cwAAAAAADG11dHVhbF9jbG9zZQAAAAMAAAAAAAAABmpvYl9pZAAAAAAABgAAAAAAAAANY2xpZW50X2Ftb3VudAAAAAAAAAsAAAAAAAAAEWZyZWVsYW5jZXJfYW1vdW50AAAAAAAACwAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAADtPcGVuIGRpc3B1dGUgb24gYSBtaWxlc3RvbmUgZHVyaW5nIHJldmlldyBvciBhZnRlciByZXZpc2lvbgAAAAAMb3Blbl9kaXNwdXRlAAAABAAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAtyZWFzb25faGFzaAAAAAPuAAAAIAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAGJBcmJpdHJhdG9yIGNhc3RzIHZvdGUgb24gZGlzcHV0ZSBwcm9wb3NpbmcgZnJlZWxhbmNlciBzaGFyZSBwZXJjZW50YWdlIGluIGJhc2lzIHBvaW50cyAoMC4uPTEwMDAwKQAAAAAADHZvdGVfZGlzcHV0ZQAAAAQAAAAAAAAABmpvYl9pZAAAAAAABgAAAAAAAAAPbWlsZXN0b25lX2luZGV4AAAAAAQAAAAAAAAACmFyYml0cmF0b3IAAAAAABMAAAAAAAAAFGZyZWVsYW5jZXJfc2hhcmVfYnBzAAAABAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAAAAAAANZ2V0X2pvYl9jb3VudAAAAAAAAAAAAAABAAAABg==",
        "AAAAAAAAAAAAAAANZ2V0X21pbGVzdG9uZQAAAAAAAAIAAAAAAAAABmpvYl9pZAAAAAAABgAAAAAAAAAPbWlsZXN0b25lX2luZGV4AAAAAAQAAAABAAAD6QAAB9AAAAAJTWlsZXN0b25lAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAAAAAAAOZ2V0X21pbGVzdG9uZXMAAAAAAAEAAAAAAAAABmpvYl9pZAAAAAAABgAAAAEAAAPpAAAD6gAAB9AAAAAJTWlsZXN0b25lAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAD5DbGllbnQgcmVjbGFpbXMgZXNjcm93IGlmIGZyZWVsYW5jZXIgbWlzc2VzIGRlYWRsaW5lIG9yIHN0YWxscwAAAAAAD3JlY2xhaW1fc3RhbGxlZAAAAAACAAAAAAAAAAZqb2JfaWQAAAAAAAYAAAAAAAAAD21pbGVzdG9uZV9pbmRleAAAAAAEAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAERSZXNvbHZlIGRpc3B1dGUgdmlhIG1ham9yaXR5IGNvbnNlbnN1cyBvciB2b3RpbmcgZGVhZGxpbmUgc2V0dGxlbWVudAAAAA9yZXNvbHZlX2Rpc3B1dGUAAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAADZTdWJtaXQgY3J5cHRvZ3JhcGhpYyBldmlkZW5jZSBoYXNoIGZvciBhbiBvcGVuIGRpc3B1dGUAAAAAAA9zdWJtaXRfZXZpZGVuY2UAAAAABAAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAAAAAAJc3VibWl0dGVyAAAAAAAAEwAAAAAAAAANZXZpZGVuY2VfaGFzaAAAAAAAA+4AAAAgAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEtDbGllbnQgcmVxdWVzdHMgYSByZXZpc2lvbiBvbiBhIHN1Ym1pdHRlZCBtaWxlc3RvbmUgKGNhcHBlZCBhdCAyIHJldmlzaW9ucykAAAAAEHJlcXVlc3RfcmV2aXNpb24AAAADAAAAAAAAAAZqb2JfaWQAAAAAAAYAAAAAAAAAD21pbGVzdG9uZV9pbmRleAAAAAAEAAAAAAAAAA1mZWVkYmFja19oYXNoAAAAAAAD7gAAACAAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAADdBZG1pbiBzbGFzaCBmb3IgcHJvdmFibHkgbWFsaWNpb3VzIG9yIGJpYXNlZCBhcmJpdHJhdG9yAAAAABBzbGFzaF9hcmJpdHJhdG9yAAAABQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAphcmJpdHJhdG9yAAAAAAATAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAGcmVhc29uAAAAAAAEAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAERTdGFrZSB0b2tlbnMgYXMgYW4gYXJiaXRyYXRvciB0byBiZSBlbGlnaWJsZSBmb3Igc2VsZWN0aW9uIGluIHBhbmVscwAAABBzdGFrZV9hcmJpdHJhdG9yAAAAAwAAAAAAAAAKYXJiaXRyYXRvcgAAAAAAEwAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAADlGcmVlbGFuY2VyIHN1Ym1pdHMgbWlsZXN0b25lIGRlbGl2ZXJhYmxlcyAoaGFzaCBvZiBmaWxlcykAAAAAAAAQc3VibWl0X21pbGVzdG9uZQAAAAMAAAAAAAAABmpvYl9pZAAAAAAABgAAAAAAAAAPbWlsZXN0b25lX2luZGV4AAAAAAQAAAAAAAAAEGRlbGl2ZXJhYmxlX2hhc2gAAAPuAAAAIAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAEBDbGllbnQgYXBwcm92ZXMgbWlsZXN0b25lIGRlbGl2ZXJhYmxlLCB0cmlnZ2VyaW5nIGluc3RhbnQgcGF5b3V0AAAAEWFwcHJvdmVfbWlsZXN0b25lAAAAAAAAAgAAAAAAAAAGam9iX2lkAAAAAAAGAAAAAAAAAA9taWxlc3RvbmVfaW5kZXgAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAAFFDbGllbnQgY2FuY2VscyBqb2IgYmVmb3JlIGZyZWVsYW5jZXIgYWNjZXB0YW5jZSAoYXBwbGllcyBjYW5jZWxsYXRpb24gZmVlIGlmIHNldCkAAAAAAAARY2FuY2VsX3VuYWNjZXB0ZWQAAAAAAAABAAAAAAAAAAZqb2JfaWQAAAAAAAYAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAEpXaXRoZHJhdyBzdGFrZWQgYXJiaXRyYXRvciBjb2xsYXRlcmFsIGlmIG5vIGFjdGl2ZSBjYXNlcyBvciBzdXJwbHVzIGV4aXN0cwAAAAAAEnVuc3Rha2VfYXJiaXRyYXRvcgAAAAAAAwAAAAAAAAAKYXJiaXRyYXRvcgAAAAAAEwAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAAAAAAAUZ2V0X2FyYml0cmF0b3Jfc3Rha2UAAAACAAAAAAAAAAphcmJpdHJhdG9yAAAAAAATAAAAAAAAAAV0b2tlbgAAAAAAABMAAAABAAAH0AAAAA9BcmJpdHJhdG9yU3Rha2UA",
        "AAAAAAAAAAAAAAAUZ2V0X2FyYml0cmF0b3Jfc3RhdHMAAAABAAAAAAAAAAphcmJpdHJhdG9yAAAAAAATAAAAAQAAB9AAAAAPQXJiaXRyYXRvclN0YXRzAA==" ]),
      options
    )
  }
  public readonly fromJSON = {
    get_job: this.txFromJSON<Result<Job>>,
        get_votes: this.txFromJSON<Array<ArbitratorVote>>,
        accept_job: this.txFromJSON<Result<void>>,
        create_job: this.txFromJSON<Result<u64>>,
        initialize: this.txFromJSON<Result<void>>,
        abandon_job: this.txFromJSON<Result<void>>,
        decline_job: this.txFromJSON<Result<void>>,
        get_dispute: this.txFromJSON<Result<Dispute>>,
        auto_release: this.txFromJSON<Result<void>>,
        get_evidence: this.txFromJSON<Array<EvidenceRecord>>,
        mutual_close: this.txFromJSON<Result<void>>,
        open_dispute: this.txFromJSON<Result<void>>,
        vote_dispute: this.txFromJSON<Result<void>>,
        get_job_count: this.txFromJSON<u64>,
        get_milestone: this.txFromJSON<Result<Milestone>>,
        get_milestones: this.txFromJSON<Result<Array<Milestone>>>,
        reclaim_stalled: this.txFromJSON<Result<void>>,
        resolve_dispute: this.txFromJSON<Result<void>>,
        submit_evidence: this.txFromJSON<Result<void>>,
        request_revision: this.txFromJSON<Result<void>>,
        slash_arbitrator: this.txFromJSON<Result<void>>,
        stake_arbitrator: this.txFromJSON<Result<void>>,
        submit_milestone: this.txFromJSON<Result<void>>,
        approve_milestone: this.txFromJSON<Result<void>>,
        cancel_unaccepted: this.txFromJSON<Result<void>>,
        unstake_arbitrator: this.txFromJSON<Result<void>>,
        get_arbitrator_stake: this.txFromJSON<ArbitratorStake>,
        get_arbitrator_stats: this.txFromJSON<ArbitratorStats>
  }
}