//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!

mod registry;

pub use registry::{ProposalRegistry, TextProposalExecutable};

use rstd::vec::Vec;

use crate::engine;
use codec::Encode;
use rstd::clone::Clone;
use srml_support::{decl_module, decl_storage};

/// 'Proposals codex' substrate module Trait
pub trait Trait: system::Trait + engine::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as ProposalCodex {}
}

decl_module! {
    /// 'Proposal codex' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Create text (signal) proposal type. On approval prints its content.
        pub fn create_text_proposal(origin, title: Vec<u8>, body: Vec<u8>) {
//			let proposer_id = ensure_signed(origin)?;

            let parameters = crate::ProposalParameters {
                voting_period: T::BlockNumber::from(3u32),
                approval_quorum_percentage: 49,
            };

            let text_proposal = TextProposalExecutable{title, body};
            let proposal_code = text_proposal.encode();

            <engine::Module<T>>::create_proposal(
                origin,
                //system::RawOrigin::Root.into(),
                parameters,
                text_proposal.proposal_type(),
                proposal_code
            )?;
        }
    }
}
