mod mock;

use super::*;
use mock::*;

/*

    fn set_balance_proposal(value: u64) -> Call {
        Call::Balances(balances::Call::set_balance(42, value, 0))
    }

*/

/*

        fn create_proposal(origin,
            proposer_id : T::AccountId,
            parameters : ProposalParameters,
            proposal_code: Box<T::ProposalCode>
        ) {

*/

#[test]
fn create_text_proposal() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::Root.into();
        //let origin = Origin::signed(1);

        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        let proposals_id =
            mock::Proposals::create_proposal(origin, 1, parameters, Box::new(text_proposal_call))
                .unwrap();

        mock::Proposals::execute_proposal(proposals_id);
    });
}

//#[test]
//fn save_and_execute_text_proposal() {
//    initial_test_ext().execute_with(|| {
//        let origin = Origin::signed(1);
//
//        let text_proposal_call = super::mock::Call::ProposalCodex(
//            super::codex::Call::create_text_proposal(b"title".to_vec(), b"body".to_vec()),
//        );
//
//        mock::Proposals::save_proposal(origin, 100, Box::new(text_proposal_call));
//
//        let origin2 = Origin::signed(1);
//        mock::Proposals::execute_proposal(origin2);
//    });
//}
