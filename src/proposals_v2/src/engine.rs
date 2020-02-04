//! Proposals engine module for the Joystream platform. Version 2.
//! Provides methods and extrinsics to create and vote for proposals.
//!
//! Supported extrinsics:
//! - vote
//! - create_proposal
//!
//! Should be added to the runtime along with Default implementation for Call:
//!

use rstd::prelude::*;

use runtime_primitives::traits::EnsureOrigin;

use srml_support::{decl_module, decl_storage, dispatch, print};

use super::*;

/// Proposals engine trait.
pub trait Trait: system::Trait + timestamp::Trait {
    /// Origin from which proposals must come.
    type ProposalOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    /// Origin from which votes must come.
    type VoteOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    /// Provides data for voting. Defines maximum voters count for the proposal.
    type TotalVotersCounter: VotersParameters;

    //    /// Proposals executable code. Requires decoding before the execution.
    //    type ProposalCode: Parameter + Default;

    type ProposalCodeDecoder: ProposalCodeDecoder;
}

// Storage for the proposals module
decl_storage! {
    trait Store for Module<T: Trait> as Proposals {
        /// Map proposal by its id.
        Proposals get(fn proposals): map u32 => Proposal<T::BlockNumber, T::AccountId>;

        /// Count of all proposals that have been created.
        pub ProposalCount get(fn proposal_count): u32;

        /// Map proposal executable code by proposal id.
        ProposalCode get(fn proposal_codes): map u32 =>  Vec<u8>;

        /// Map votes by proposal id.
        VotesByProposalId get(fn votes_by_proposal): map u32 => Vec<Vote<T::AccountId>>;

        /// Ids of proposals that are open for voting (have not been finalized yet).
        pub ActiveProposalIds get(fn active_proposal_ids): Vec<u32> = vec![];

        /// Proposal tally results map
        pub(crate) TallyResults get(fn tally_results): map u32 => TallyResult<T::BlockNumber>;
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
            let tally_results = Self::tally();

            // mutation

            for  tally_result in tally_results {
                <TallyResults<T>>::insert(tally_result.proposal_id, &tally_result);

                let update_proposal_result = Self::update_proposal_status(
                    tally_result.proposal_id,
                    tally_result.status
                );

                if let Err(e) = update_proposal_result {
                    print(e);
                }
            }
        }
    }
}

impl<T: Trait> Module<T> {
    // TODO: introduce own error types
    /// Create proposal. Requires root permissions.
    pub fn create_proposal(
        origin: T::Origin,
        //        proposer_id: T::AccountId,
        parameters: ProposalParameters<T::BlockNumber>,
        proposal_type: u32,
        proposal_code: Vec<u8>,
    ) -> Result<(), &'static str> {
        //        ensure_root(origin)?;
        let proposer_id = T::ProposalOrigin::ensure_origin(origin)?; //TODO

        let next_proposal_count_value = Self::proposal_count() + 1;
        let new_proposal_id = next_proposal_count_value;

        let new_proposal = Proposal {
            created: Self::current_block(),
            parameters,
            proposer_id,
            proposal_type,
            status: ProposalStatus::Active,
        };

        // mutation
        <Proposals<T>>::insert(new_proposal_id, new_proposal);
        <ProposalCode>::insert(new_proposal_id, proposal_code);
        ActiveProposalIds::mutate(|ids| ids.push(new_proposal_id));
        ProposalCount::put(next_proposal_count_value);

        Ok(())
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
        //let origin = system::RawOrigin::Root.into();
        let proposal = Self::proposals(proposal_id);
        let proposal_code = Self::proposal_codes(proposal_id);

        let proposal =
            T::ProposalCodeDecoder::decode_proposal(proposal.proposal_type, proposal_code);

        let _result = proposal.unwrap().execute();

        //        if let Err(e) = result {
        //            let e: DispatchError = e.into();
        //            print(e);
        //        };
    }

    /// Voting results tally.
    /// Returns proposals with changed status and tally results
    fn tally() -> Vec<TallyResult<T::BlockNumber>> {
        let mut results = Vec::new();
        for &proposal_id in Self::active_proposal_ids().iter() {
            let votes = Self::votes_by_proposal(proposal_id);
            let proposal = Self::proposals(proposal_id);

            if let Some(tally_result) = proposal.tally_results(
                proposal_id,
                votes,
                T::TotalVotersCounter::total_voters_count(),
                Self::current_block(),
            ) {
                results.push(tally_result);
            }
        }

        results
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

            let mut new_active_ids = other_active_ids;
            match new_status {
                ProposalStatus::Rejected | ProposalStatus::Expired => Self::reject_proposal(pid)?,
                ProposalStatus::Approved => Self::approve_proposal(pid)?,
                ProposalStatus::Active => {
                    // return back active proposal id
                    new_active_ids.push(pid);
                }
            }
            ActiveProposalIds::put(new_active_ids);
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
