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

    /// Temporary field which defines expected quorum votes count. Used by quorum calculation
    /// Will be changed to percentage
    pub temp_quorum_vote_count: u32,

    //    /// Temporary field which defines expected threshold to pass the vote.
    //    /// Will be changed to percentage
    //    pub temp_threshold_vote_count: u32,
    /// Temporary field which defines total expected votes count. Used by quorum calculation
    /// Will be changed to some kind of votes manager
    pub temp_total_vote_count: u32,
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
    /// Current proposal status
    pub status: ProposalStatus,
}

impl<BlockNumber: Add<Output = BlockNumber> + PartialOrd + Copy, AccountId>
    Proposal<BlockNumber, AccountId>
{
    /// Returns whether voting period expired by now
    pub fn is_voting_period_expired(&self, now: BlockNumber) -> bool {
        now >= self.created + self.parameters.voting_period
    }

    /// Voting results tally for single proposal.
    /// Parameters: own proposal id, current time, votes.
    /// Returns tally results if proposal status will should change
    pub fn tally_results(
        self,
        proposal_id: u32,
        votes: Vec<Vote<AccountId>>,
        now: BlockNumber,
    ) -> Option<TallyResult<BlockNumber>> {
        let mut abstentions: u32 = 0;
        let mut approvals: u32 = 0;
        let mut rejections: u32 = 0;

        for vote in votes.iter() {
            match vote.vote_kind {
                VoteKind::Abstain => abstentions += 1,
                VoteKind::Approve => approvals += 1,
                VoteKind::Reject => rejections += 1,
            }
        }

        let is_expired = self.is_voting_period_expired(now);

        let votes_count = votes.len() as u32;
        let all_voted = votes_count == self.parameters.temp_total_vote_count;

        let quorum_reached = votes_count >= self.parameters.temp_quorum_vote_count
            && approvals >= self.parameters.temp_quorum_vote_count;

        let new_status: Option<ProposalStatus> = if quorum_reached {
            Some(ProposalStatus::Approved)
        } else if is_expired {
            // Proposal has been expired and quorum not reached.
            Some(ProposalStatus::Expired)
        } else if all_voted {
            Some(ProposalStatus::Rejected)
        } else {
            None
        };

        if let Some(status) = new_status {
            Some(TallyResult {
                proposal_id,
                abstentions,
                approvals,
                rejections,
                status,
                finalized_at: now,
            })
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn proposal_voting_period_expired() {
        let mut proposal = Proposal::<u64, u64>::default();

        proposal.created = 1;
        proposal.parameters.voting_period = 3;

        assert!(proposal.is_voting_period_expired(4));
    }

    #[test]
    fn proposal_voting_period_not_expired() {
        let mut proposal = Proposal::<u64, u64>::default();

        proposal.created = 1;
        proposal.parameters.voting_period = 3;

        assert!(!proposal.is_voting_period_expired(3));
    }

    #[test]
    fn tally_results_proposal_expired() {
        let mut proposal = Proposal::<u64, u64>::default();
        let proposal_id = 1;
        let now = 5;
        proposal.created = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.temp_quorum_vote_count = 3;
        proposal.parameters.temp_total_vote_count = 5;

        let votes = vec![
            Vote {
                voter_id: 1,
                vote_kind: VoteKind::Approve,
            },
            Vote {
                voter_id: 2,
                vote_kind: VoteKind::Approve,
            },
            Vote {
                voter_id: 4,
                vote_kind: VoteKind::Reject,
            },
        ];

        let expected_tally_results = TallyResult {
            proposal_id,
            abstentions: 0,
            approvals: 2,
            rejections: 1,
            status: ProposalStatus::Expired,
            finalized_at: now
        };

        assert_eq!(proposal.tally_results(proposal_id, votes, now), Some(expected_tally_results));
    }
    #[test]
    fn tally_results_proposal_approved() {
        let mut proposal = Proposal::<u64, u64>::default();
        let proposal_id = 1;
        proposal.created = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.temp_quorum_vote_count = 3;
        proposal.parameters.temp_total_vote_count = 5;

        let votes = vec![
            Vote {
                voter_id: 1,
                vote_kind: VoteKind::Approve,
            },
            Vote {
                voter_id: 2,
                vote_kind: VoteKind::Approve,
            },
            Vote {
                voter_id: 3,
                vote_kind: VoteKind::Approve,
            },
            Vote {
                voter_id: 4,
                vote_kind: VoteKind::Reject,
            },
        ];

        let expected_tally_results = TallyResult {
            proposal_id,
            abstentions: 0,
            approvals: 3,
            rejections: 1,
            status: ProposalStatus::Approved,
            finalized_at: 2
        };

        assert_eq!(proposal.tally_results(proposal_id, votes, 2), Some(expected_tally_results));
    }

    #[test]
    fn tally_results_proposal_rejected() {
        let mut proposal = Proposal::<u64, u64>::default();
        let proposal_id = 1;
        let now = 2;

        proposal.created = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.temp_quorum_vote_count = 3;
        proposal.parameters.temp_total_vote_count = 4;

        let votes = vec![
            Vote {
                voter_id: 1,
                vote_kind: VoteKind::Reject,
            },
            Vote {
                voter_id: 2,
                vote_kind: VoteKind::Reject,
            },
            Vote {
                voter_id: 3,
                vote_kind: VoteKind::Abstain,
            },
            Vote {
                voter_id: 4,
                vote_kind: VoteKind::Approve,
            },
        ];

        let expected_tally_results = TallyResult {
            proposal_id,
            abstentions: 1,
            approvals: 1,
            rejections: 2,
            status: ProposalStatus::Rejected,
            finalized_at: now
        };

        assert_eq!(proposal.tally_results(proposal_id, votes, now), Some(expected_tally_results));
    }

   #[test]
    fn tally_results_are_empty_with_not_expired_voting_period() {
        let mut proposal = Proposal::<u64, u64>::default();
        let proposal_id = 1;
        let now = 2;

        proposal.created = 1;
        proposal.parameters.voting_period = 3;
        proposal.parameters.temp_quorum_vote_count = 3;
        proposal.parameters.temp_total_vote_count = 4;

        let votes = vec![
            Vote {
                voter_id: 1,
                vote_kind: VoteKind::Abstain,
            },
        ];


        assert_eq!(proposal.tally_results(proposal_id, votes, now), None);
    }
}
