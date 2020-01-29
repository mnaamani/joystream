//! Proposals engine module for the Joystream platform. Version 2.
//! Provides methods and extrinsics to create and vote for proposals.
//!
//! Supported extrinsics:
//! - vote
//!
//! Public API:
//! - create_proposal

use rstd::boxed::Box;
use rstd::fmt::Debug;
use rstd::prelude::*;

use runtime_primitives::traits::{Dispatchable, EnsureOrigin};
use runtime_primitives::DispatchError;

use srml_support::{decl_module, decl_storage, dispatch, print, Parameter};

use super::*;

/// Proposals engine trait.
pub trait Trait: system::Trait + timestamp::Trait {
    /// Proposals executable code. Can be instantiated by external module Call enum members.
    type ProposalCode: Parameter + Dispatchable<Origin = Self::Origin> + Default;

    /// Origin from which proposals must come.
    type ProposalOrigin: EnsureOrigin<Self::Origin, Success = ()>;

    /// Origin from which votes must come.
    type VoteOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    // Calculates quorum and vote threshold
    //type QuorumProvider: QuorumProvider;
}

// Storage for the proposals module
decl_storage! {
    trait Store for Module<T: Trait> as Proposals {
        /// Map proposal by its id.
        Proposals get(fn proposals): map u32 => Proposal<T::BlockNumber, T::AccountId>;

        /// Count of all proposals that have been created.
        ProposalCount get(fn proposal_count): u32;

        /// Map proposal executable code by proposal id.
        ProposalCode get(fn proposal_codes): map u32 =>  T::ProposalCode;

        /// Map votes by proposal id.
        VotesByProposalId get(fn votes_by_proposal): map u32 => Vec<Vote<T::AccountId>>;

        /// Ids of proposals that are open for voting (have not been finalized yet).
        ActiveProposalIds get(fn active_proposal_ids): Vec<u32> = vec![];

        /// Proposal tally results map
        pub TallyResults get(fn tally_results): map u32 => TallyResult<T::BlockNumber>;
    }
}

decl_module! {
    /// 'Proposal engine' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Vote extrinsic. Conditions:  origin must allow votes.
        pub fn vote(origin, proposal_id: u32, vote: VoteKind) -> dispatch::Result {
            let voter_id = T::VoteOrigin::ensure_origin(origin)?;

            Self::process_vote(voter_id, proposal_id, vote)
        }

        /// Block finalization. Perform voting period check and vote result tally.
        fn on_finalize(_n: T::BlockNumber) {
            if let Err(e) = Self::tally() {
                print(e);
            }
        }
    }
}

/// Defines an error during 'create_proposal' invocation
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateProposalError {}

impl<T: Trait> Module<T> {
    // TODO: introduce own error types
    /// Create proposal. Requires root permissions.
    pub fn create_proposal(
        origin: T::Origin,
        proposer_id: T::AccountId,
        parameters: ProposalParameters<T::BlockNumber>,
        proposal_code: Box<T::ProposalCode>,
    ) -> Result<u32, CreateProposalError> {
        T::ProposalOrigin::ensure_origin(origin).map_err(|_| CreateProposalError {})?;

        let next_proposal_count_value = Self::proposal_count() + 1;
        let new_proposal_id = next_proposal_count_value;

        let new_proposal = Proposal {
            created: Self::current_block(),
            parameters,
            proposer_id,
            status: ProposalStatus::Active,
        };

        // mutation
        <Proposals<T>>::insert(new_proposal_id, new_proposal);
        <ProposalCode<T>>::insert(new_proposal_id, proposal_code);
        ActiveProposalIds::mutate(|ids| ids.push(new_proposal_id));
        ProposalCount::put(next_proposal_count_value);

        Ok(new_proposal_id)
    }
}

impl<T: Trait> Module<T> {
    // Wrapper-function over system::block_number()
    fn current_block() -> T::BlockNumber {
        <system::Module<T>>::block_number()
    }

    // Actual vote processor. Stores votes for proposal.
    fn process_vote(voter_id: T::AccountId, proposal_id: u32, vote: VoteKind) -> dispatch::Result {
        let new_vote = Vote {
            voter_id,
            vote_kind: vote,
        };
        if <VotesByProposalId<T>>::exists(proposal_id) {
            // Append a new vote to other votes on this proposal:
            <VotesByProposalId<T>>::mutate(proposal_id, |votes| votes.push(new_vote));
        } else {
            // This is the first vote on this proposal:
            <VotesByProposalId<T>>::insert(proposal_id, vec![new_vote]);
        }
        Ok(())
    }

    // Executes approved proposal code
    fn execute_proposal(proposal_id: u32) {
        //let origin = system::RawOrigin::None.into();
        let origin = system::RawOrigin::Root.into();
        let proposal = Self::proposal_codes(proposal_id);

        let result = proposal.dispatch(origin);

        if let Err(e) = result {
            let e: DispatchError = e.into();
            print(e);
        };
    }

    /// Voting results tally
    fn tally() -> dispatch::Result {
        let quorum: u32 = 1;

        for &proposal_id in Self::active_proposal_ids().iter() {
            let votes = Self::votes_by_proposal(proposal_id);
            let proposal = Self::proposals(proposal_id);

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

            let is_expired = proposal.is_voting_period_expired(Self::current_block());

            let quorum_reached = quorum > 0 && approvals >= quorum;

            let new_status: Option<ProposalStatus> = if quorum_reached {
                Some(ProposalStatus::Approved)
            } else {
                if is_expired {
                    // Proposal has been expired and quorum not reached.
                    Some(ProposalStatus::Expired)
                } else {
                    None
                }
            };

            // Some(ProposalStatus::Rejected)

            // TODO move next block outside of tally to 'end_block'
            if let Some(status) = new_status {
                Self::update_proposal_status(proposal_id, status.clone())?;
                let tally_result = TallyResult {
                    proposal_id,
                    abstentions,
                    approvals,
                    rejections,
                    status,
                    finalized_at: Self::current_block(),
                };
                <TallyResults<T>>::insert(proposal_id, &tally_result);
            }
        }

        Ok(())
    }

    /// Updates proposal status and removes proposal from active ids.
    fn update_proposal_status(proposal_id: u32, new_status: ProposalStatus) -> dispatch::Result {
        let all_active_ids = Self::active_proposal_ids();
        let all_len = all_active_ids.len();
        let other_active_ids: Vec<u32> = all_active_ids
            .into_iter()
            .filter(|&id| id != proposal_id)
            .collect();

        let not_found_in_active = other_active_ids.len() == all_len;
        if not_found_in_active {
            // Seems like this proposal's status has been updated and removed from active.
            Err("MSG_PROPOSAL_STATUS_ALREADY_UPDATED")
        } else {
            let pid = proposal_id.clone();
            match new_status {
                ProposalStatus::Rejected | ProposalStatus::Expired => Self::reject_proposal(pid)?,
                ProposalStatus::Approved => Self::approve_proposal(pid)?,
                ProposalStatus::Active => { /* nothing */ }
            }
            ActiveProposalIds::put(other_active_ids);
            <Proposals<T>>::mutate(proposal_id, |p| p.status = new_status.clone());
            Ok(())
        }
    }

    /// Reject a proposal. The staked deposit will be returned to a proposer.
    fn reject_proposal(_proposal_id: u32) -> dispatch::Result {
        Ok(())
    }

    /// Approve a proposal. The staked deposit will be returned.
    fn approve_proposal(proposal_id: u32) -> dispatch::Result {
        Self::execute_proposal(proposal_id);

        Ok(())
    }
}
