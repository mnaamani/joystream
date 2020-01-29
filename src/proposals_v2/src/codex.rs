//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!

use rstd::str::from_utf8;
use rstd::vec::Vec;

use srml_support::{decl_module, decl_storage, print};
use system::ensure_root;

/// 'Proposals codex' substrate module Trait
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as ProposalCodex {}
}

decl_module! {
    /// 'Proposal codex' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Text signal proposal type. On approval prints its content.
        fn create_text_proposal(origin, title: Vec<u8>, body: Vec<u8>) {
            ensure_root(origin)?;

            print("Proposal: ");
            print(from_utf8(title.as_slice()).unwrap());
            print("Description:");
            print(from_utf8(body.as_slice()).unwrap());
        }
    }
}
