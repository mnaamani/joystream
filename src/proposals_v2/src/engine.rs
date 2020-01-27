use rstd::boxed::Box;
use rstd::prelude::*;

use runtime_primitives::traits::{Dispatchable, DispatchResult};
use runtime_primitives::{DispatchError};

use srml_support::{
    //    decl_event,
    decl_module,
    decl_storage, //dispatch, ensure
    Parameter,
};

//use system::{ensure_root};

//use crate::currency::{BalanceOf, GovernanceCurrency};

use super::*;

pub trait Trait: system::Trait + timestamp::Trait {
    /// The overarching event type.
    //type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type ProposalCode: Parameter + Dispatchable<Origin = Self::Origin> + Default;
}

/*

use rstd::vec::Vec;

Proposals get(fn proposals): Vec<T::Proposal>

<Proposals<T>>::append_or_put(&[*proposal][..]);

*/

decl_storage! {
    trait Store for Module<T: Trait> as Proposals {
        /// Map proposal by its id.
        Proposals get(fn proposals): map u32 => Proposal<T::BlockNumber, T::AccountId>;

        /// Count of all proposals that have been created.
        ProposalCount get(fn proposal_count): u32;

        /// Map proposal executable code by proposal id.
        ProposalCode get(fn proposal_codes): map u32 =>  T::ProposalCode;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

//        /// Create proposal. Requires root permissions.
//        fn create_proposal(origin,
//            proposer_id : T::AccountId,
//            parameters : ProposalParameters,
//            proposal_code: Box<T::ProposalCode>
//        ) {
//            ensure_root(origin)?;
//
//            let next_proposal_count_value = Self::proposal_count() + 1;
//            let new_proposal_id = next_proposal_count_value;
//
//            let new_proposal = Proposal {
//                created : Self::current_block(),
//                parameters,
//                proposer_id
//            };
//
//            // mutation
//            <Proposals<T>>::insert(new_proposal_id, new_proposal);
//            <ProposalCode<T>>::insert(new_proposal_id, proposal_code);
//            ProposalCount::put(next_proposal_count_value);
//        }
    }
}

//pub type ProposalsDispatchResult = DispatchResult<DispatchError>;
//pub type SystemResult = Result<(), system::Error>;
//
//impl From<Result<(), system::Error>> for ProposalsDispatchResult {
//    fn from(_: SystemResult) -> Self {
//        Err(DispatchError::new(None, 0, Some("Insufficient rights")))
//    }
//}

impl<T: Trait> Module<T> {
    // Wrapper-function over system::block_number()
    fn current_block() -> T::BlockNumber {
        <system::Module<T>>::block_number()
    }

    pub fn execute_proposal() {
        let origin = system::RawOrigin::Root.into();

        let proposal = Self::proposal_codes(1);
        proposal.dispatch(origin);
    }

    // TODO: introduce own error types
    // Create proposal. Requires root permissions.
    pub fn create_proposal(origin: T::Origin,
                           proposer_id: T::AccountId,
                           parameters: ProposalParameters,
                           proposal_code: Box<T::ProposalCode>
    ) -> DispatchResult<DispatchError> {
//        ensure_root(origin)?;

        let next_proposal_count_value = Self::proposal_count() + 1;
        let new_proposal_id = next_proposal_count_value;

        let new_proposal = Proposal {
            created: Self::current_block(),
            parameters,
            proposer_id
        };

        // mutation
        <Proposals<T>>::insert(new_proposal_id, new_proposal);
        <ProposalCode<T>>::insert(new_proposal_id, proposal_code);
        ProposalCount::put(next_proposal_count_value);

        Ok(())
    }
}
