//! Proposals system for the Joystream platform. Version 2.
//! Provides modules to create and vote for proposals.
//!
//! Modules:
//! - engine - provides public API and extrinsics to create and vote for proposals.
//! - codex - contains preset proposal types.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

pub mod codex;
pub mod engine;
mod types;

#[cfg(test)]
mod tests;

pub use codex::*;
pub use engine::*;

pub use types::TallyResult;
pub use types::{Proposal, ProposalParameters, ProposalStatus};
pub use types::{ProposalCodeDecoder, ProposalExecutable};
pub use types::{Vote, VoteKind, VotersParameters};
