//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

pub use proposal_types::{ProposalType, TextProposalExecutable};

mod proposal_types;
#[cfg(test)]
mod tests;

use codec::Encode;
use proposal_engine::*;
use rstd::clone::Clone;
use rstd::vec::Vec;
use srml_support::decl_module;

/// 'Proposals codex' substrate module Trait
pub trait Trait: system::Trait + proposal_engine::Trait {}

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

            <proposal_engine::Module<T>>::create_proposal(
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
