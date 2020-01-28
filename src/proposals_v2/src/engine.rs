//! Proposals engine module for the Joystream platform. Version 2.
//! Provides methods and extrinsics to create and vote for proposals.
//!
//! Supported extrinsics:
//! - vote
//!
//! Public API:
//! - create_proposal

use rstd::boxed::Box;
use rstd::prelude::*;

use runtime_primitives::traits::{ Dispatchable, EnsureOrigin};

use srml_support::{
    dispatch,
    decl_module,
    decl_storage,
    Parameter,
};


use super::*;

/// Proposals engine trait.
pub trait Trait: system::Trait + timestamp::Trait {
    /// The overarching event type.
    //type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Proposals executable code. Can be instantiated by external module Call enum members.
    type ProposalCode: Parameter + Dispatchable<Origin = Self::Origin> + Default;

    /// Origin from which proposals must come.
    type ProposalOrigin: EnsureOrigin<Self::Origin, Success=()>;

    /// Origin from which votes must come.
    type VoteOrigin: EnsureOrigin<Self::Origin, Success=Self::AccountId>;
}

/*

use rstd::vec::Vec;

Proposals get(fn proposals): Vec<T::Proposal>

<Proposals<T>>::append_or_put(&[*proposal][..]);

*/

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
    }
}


/*

 fn vote_on_proposal(origin, proposal_id: u32, vote: VoteKind) {
            let voter = ensure_signed(origin)?;
            ensure!(Self::is_councilor(&voter), MSG_ONLY_COUNCILORS_CAN_VOTE);

            ensure!(<Proposals<T>>::exists(proposal_id), MSG_PROPOSAL_NOT_FOUND);
            let proposal = Self::proposals(proposal_id);

            ensure!(proposal.status == Active, MSG_PROPOSAL_FINALIZED);

            let not_expired = !Self::is_voting_period_expired(proposal.proposed_at);
            ensure!(not_expired, MSG_PROPOSAL_EXPIRED);

            let did_not_vote_before = !<VoteByAccountAndProposal<T>>::exists((voter.clone(), proposal_id));
            ensure!(did_not_vote_before, MSG_YOU_ALREADY_VOTED);

            Self::_process_vote(voter, proposal_id, vote)?;
        }

*/

decl_module! {
    /// 'Proposal engine' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        /// Vote extrinsic. Conditions:  origin must allow votes.
        fn vote(origin, proposal_id: u32, vote: VoteKind) -> dispatch::Result {
            let voter_id = T::VoteOrigin::ensure_origin(origin)?;

            Self::process_vote(voter_id, proposal_id, vote)
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
        parameters: ProposalParameters,
        proposal_code: Box<T::ProposalCode>,
    ) -> Result<u32, CreateProposalError> {
        T::ProposalOrigin::ensure_origin(origin).map_err(|_| CreateProposalError {})?;

        let next_proposal_count_value = Self::proposal_count() + 1;
        let new_proposal_id = next_proposal_count_value;

        let new_proposal = Proposal {
            created: Self::current_block(),
            parameters,
            proposer_id,
        };

        // mutation
        <Proposals<T>>::insert(new_proposal_id, new_proposal);
        <ProposalCode<T>>::insert(new_proposal_id, proposal_code);
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
        let new_vote = Vote{voter_id, vote_kind: vote};
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
    fn execute_proposal(proposal_id: u32){
        let origin = system::RawOrigin::Root.into();
        let proposal = Self::proposal_codes(proposal_id);

        let _ = proposal.dispatch(origin);
    }
}