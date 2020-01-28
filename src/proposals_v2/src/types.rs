//! Proposals types module for the Joystream platform. Version 2.
//! Provides types for the proposal engine.

use codec::{Decode, Encode};
use rstd::cmp::PartialOrd;
use rstd::ops::Add;
use rstd::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Current status of the proposal
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum ProposalStatus {
    /// A new proposal that is available for voting.
    Active,

    /// To clear the quorum requirement, the percentage of council members with revealed votes
    /// must be no less than the quorum value for the given proposal type.
    Approved,

    /// A proposal was rejected
    Rejected,

    /// Not enough votes and voting period expired.
    Expired,
}

impl Default for ProposalStatus {
    fn default() -> Self {
        ProposalStatus::Active
    }
}

/// Vote kind for the proposal. Sum of all votes defines proposal status.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum VoteKind {
    /// Pass, an alternative or a ranking, for binary, multiple choice
    /// and ranked choice propositions, respectively.
    Approve,

    /// Against proposal.
    Reject,

    /// Signals presence, but unwillingness to cast judgment on substance of vote.
    Abstain,
}

impl Default for VoteKind {
    fn default() -> Self {
        VoteKind::Reject
    }
}

/// Proposal parameters required to manage proposal risk.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct ProposalParameters<BlockNumber> {
    /// During this period, votes can be accepted
    pub voting_period: BlockNumber,
    //pub stake: BalanceOf<T>, //<T: GovernanceCurrency>
}

/// 'Proposal' contains information necessary for the proposal system functioning.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Proposal<BlockNumber, AccountId> {
    /// Proposals parameter, characterize different proposal types.
    pub parameters: ProposalParameters<BlockNumber>,

    /// Identifier of member proposing.
    pub proposer_id: AccountId,

    /// When it was created.
    pub created: BlockNumber,
    // Any stake associated with the proposal.
    //pub stake: Option<BalanceOf<T>>

    //Stage: One among the following.
    pub status: ProposalStatus,
}

impl<BlockNumber: Add<Output = BlockNumber> + PartialOrd + Copy, AccountId>
    Proposal<BlockNumber, AccountId>
{
    pub fn is_voting_period_expired(&self, now: BlockNumber) -> bool {
        now >= self.created + self.parameters.voting_period
    }
}

/// Vote. Characterized by voter and vote kind.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct Vote<AccountId> {
    /// Origin of the vote
    pub voter_id: AccountId,

    /// Vote kind
    pub vote_kind: VoteKind,
}

/// Tally result for the proposal
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct TallyResult<BlockNumber> {
    /// Proposal Id
    pub proposal_id: u32,

    /// 'Abstention' votes count
    pub abstentions: u32,

    /// 'Approve' votes count
    pub approvals: u32,

    /// 'Reject' votes count
    pub rejections: u32,

    /// Proposal status after tally
    pub status: ProposalStatus,

    /// Proposal finalization block number
    pub finalized_at: BlockNumber,
}
