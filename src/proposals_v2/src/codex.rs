use rstd::vec::Vec;

#[derive(Debug)]
pub struct TextProposal {
    pub title: Vec<u8>,
    pub body: Vec<u8>,
}

use srml_support::{decl_module, decl_storage, print};

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as ProposalCodex {
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_text_proposal(origin, title: Vec<u8>, body: Vec<u8>) {
                print("text");
        }
    }
}
