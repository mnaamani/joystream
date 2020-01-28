//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!


use rstd::vec::Vec;

use srml_support::{decl_module, decl_storage, print};

/// 'Proposals codex' substrate module Trait
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as ProposalCodex {
        // Declare storage and getter functions here
    }
}

decl_module! {
	/// 'Proposal codex' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Text signal proposal type. On approval prints its content.
        fn create_text_proposal(_origin, _title: Vec<u8>, _body: Vec<u8>) {
                print("text");
        }
    }
}
