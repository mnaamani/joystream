//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!

mod proposal_types;

pub use proposal_types::FaultyExecutable;
pub use proposal_types::{ProposalType, TextProposalExecutable};

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
            let parameters = crate::ProposalParameters {
                voting_period: T::BlockNumber::from(3u32),
                approval_quorum_percentage: 49,
            };

            let text_proposal = TextProposalExecutable{
            	title: title.clone(),
            	 body: body.clone()
           	};
            let proposal_code = text_proposal.encode();

            <engine::Module<T>>::create_proposal(
                origin,
                parameters,
     			title,
                body,
                text_proposal.proposal_type(),
                proposal_code
            )?;
        }
    }
}
