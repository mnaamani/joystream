mod mock;

use super::*;
use mock::*;

/*

    fn set_balance_proposal(value: u64) -> Call {
        Call::Balances(balances::Call::set_balance(42, value, 0))
    }

*/

#[test]
fn create_text_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::Root.into();

        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        let _proposals_id =
            mock::Proposals::create_proposal(origin, 1, parameters, Box::new(text_proposal_call))
                .unwrap();
    });
}

#[test]
fn create_text_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::None.into();

        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        assert!(mock::Proposals::create_proposal(
            origin,
            1,
            parameters,
            Box::new(text_proposal_call)
        )
        .is_err());
    });
}
