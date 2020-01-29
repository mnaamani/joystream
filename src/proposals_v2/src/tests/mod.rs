mod mock;

use super::*;
use mock::*;

use crate::engine::*;

use srml_support::{ StorageMap};
use runtime_primitives::traits::{OnFinalize, OnInitialize};

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <Proposals as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <Proposals as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

fn run_to_block_and_finalize(n: u64) {
    run_to_block(n);
    <Proposals as OnFinalize<u64>>::on_finalize(n);
}

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
            Proposals::create_proposal(origin, 1, parameters, Box::new(text_proposal_call))
                .unwrap();
    });
}

#[test]
fn create_text_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let origin = system::RawOrigin::None.into();
        let parameters = ProposalParameters { voting_period: 3 };
        assert_eq!(
            Proposals::create_proposal(origin, 1, parameters, Box::new(text_proposal_call)),
            Err(CreateProposalError{})
        );
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        let proposals_id = Proposals::create_proposal(
            system::RawOrigin::Root.into(),
            1,
            parameters,
            Box::new(text_proposal_call),
        )
        .unwrap();

        assert_eq!(
            Proposals::vote(
                system::RawOrigin::Signed(1).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );
    });
}

#[test]
fn vote_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        assert_eq!(
            Proposals::vote(system::RawOrigin::None.into(), 1, VoteKind::Approve),
            Err("Invalid origin")
        );
    });
}



#[test]
fn proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        let proposals_id = Proposals::create_proposal(
            system::RawOrigin::Root.into(),
            1,
            parameters,
            Box::new(text_proposal_call),
        )
            .unwrap();

        assert_eq!(
            Proposals::vote(
                system::RawOrigin::Signed(1).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );

        run_to_block_and_finalize(2);
    });
}

#[test]
fn tally_calculation_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal_call = mock::Call::ProposalCodex(codex::Call::create_text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
        ));

        let parameters = ProposalParameters { voting_period: 3 };
        let proposals_id = Proposals::create_proposal(
            system::RawOrigin::Root.into(),
            1,
            parameters,
            Box::new(text_proposal_call),
        )
            .unwrap();

        assert!(
            Proposals::vote(
                system::RawOrigin::Signed(1).into(),
                proposals_id,
                VoteKind::Approve
            ).is_ok()
        );

        assert!(
            Proposals::vote(
                system::RawOrigin::Signed(2).into(),
                proposals_id,
                VoteKind::Approve
            ).is_ok()
        );

        assert!(
            Proposals::vote(
                system::RawOrigin::Signed(3).into(),
                proposals_id,
                VoteKind::Reject
            ).is_ok()
        );

        assert!(
            Proposals::vote(
                system::RawOrigin::Signed(4).into(),
                proposals_id,
                VoteKind::Abstain
            ).is_ok()
        );

        run_to_block_and_finalize(2);

        let tally_result = <TallyResults<Test>>::get(proposals_id);

        assert_eq!(tally_result, TallyResult{
            proposal_id: proposals_id,
            abstentions: 1,
            approvals: 2,
            rejections: 1,
            status: ProposalStatus::Approved,
            finalized_at: 1
        })
    });
}