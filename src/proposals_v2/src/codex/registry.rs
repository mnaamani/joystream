use codec::{Decode, Encode};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rstd::convert::TryFrom;
use rstd::prelude::*;

use rstd::str::from_utf8;
use srml_support::print;

use crate::{ProposalCodeDecoder, ProposalExecutable};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ProposalType {
    Dummy = 0,
    Text = 1,
}

impl ProposalType {
    fn compose_executable(
        &self,
        proposal_data: Vec<u8>,
    ) -> Result<Box<dyn ProposalExecutable>, &'static str> {
        match self {
            ProposalType::Dummy => DummyExecutable::decode(&mut &proposal_data[..])
                .map_err(|err| err.what())
                .map(|obj| Box::new(obj) as Box<dyn ProposalExecutable>),
            ProposalType::Text => TextProposalExecutable::decode(&mut &proposal_data[..])
                .map_err(|err| err.what())
                .map(|obj| Box::new(obj) as Box<dyn ProposalExecutable>),
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
pub struct DummyExecutable {
    data: Vec<u8>,
}

impl ProposalExecutable for DummyExecutable {
    fn execute(&self) {
        print("dummy");
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
pub struct TextProposalExecutable {
    pub title: Vec<u8>,
    pub body: Vec<u8>,
}

impl TextProposalExecutable {
    pub fn proposal_type(&self) -> u32 {
        ProposalType::Text.into()
    }
}

impl ProposalExecutable for TextProposalExecutable {
    fn execute(&self) {
        print("Proposal: ");
        print(from_utf8(self.title.as_slice()).unwrap());
        print("Description:");
        print(from_utf8(self.body.as_slice()).unwrap());
    }
}

pub struct ProposalRegistry;
impl ProposalCodeDecoder for ProposalRegistry {
    fn decode_proposal(
        proposal_type: u32,
        proposal_code: Vec<u8>,
    ) -> Result<Box<dyn ProposalExecutable>, &'static str> {
        let result =
            ProposalType::try_from(proposal_type).map_err(|_| "Unsupported proposal type")?;

        result.compose_executable(proposal_code)
    }
}

impl ProposalRegistry {
    pub fn call() {
        let exec = DummyExecutable { data: Vec::new() };

        let code = exec.encode();

        Self::decode_proposal(1, code.to_vec()).unwrap();
    }
}
