mod mock;

use crate::*;
use mock::*;

use codec::Encode;
use rstd::collections::btree_set::BTreeSet;
use runtime_primitives::traits::{OnFinalize, OnInitialize};
use srml_support::{StorageMap, StorageValue};

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <ProposalsEngine as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <ProposalsEngine as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

fn run_to_block_and_finalize(n: u64) {
    run_to_block(n);
    <ProposalsEngine as OnFinalize<u64>>::on_finalize(n);
}

#[test]
fn create_text_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::Signed(1).into();

        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_ok());
    });
}

#[test]
fn create_text_proposal_codex_call_succeeds() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::Signed(1).into();

        assert!(
            ProposalCodex::create_text_proposal(origin, b"title".to_vec(), b"body".to_vec(),)
                .is_ok()
        );
    });
}

#[test]
fn create_text_proposal_codex_call_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let origin = system::RawOrigin::None.into();

        assert!(
            ProposalCodex::create_text_proposal(origin, b"title".to_vec(), b"body".to_vec(),)
                .is_err()
        );
    });
}

#[test]
fn create_text_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let origin = system::RawOrigin::None.into();
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };
        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_err());
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        let origin = system::RawOrigin::Signed(1).into();
        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_ok());

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        assert_eq!(
            ProposalsEngine::vote(
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
            ProposalsEngine::vote(system::RawOrigin::None.into(), 1, VoteKind::Approve),
            Err("Invalid origin")
        );
    });
}

#[test]
fn proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        let origin = system::RawOrigin::Signed(1).into();
        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_ok());

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(1).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );
        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(2).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );
        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(3).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );
        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(4).into(),
                proposals_id,
                VoteKind::Approve
            ),
            Ok(())
        );

        run_to_block_and_finalize(2);

        let proposal = <crate::engine::Proposals<Test>>::get(proposals_id);

        assert_eq!(
            proposal,
            Proposal {
                proposal_type: 1,
                parameters,
                proposer_id: 1,
                created: 1,
                status: ProposalStatus::Executed
            }
        )
    });
}

#[test]
fn tally_calculation_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 49,
        };

        let origin = system::RawOrigin::Signed(1).into();
        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_ok());

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        assert!(ProposalsEngine::vote(
            system::RawOrigin::Signed(1).into(),
            proposals_id,
            VoteKind::Approve
        )
        .is_ok());

        assert!(ProposalsEngine::vote(
            system::RawOrigin::Signed(2).into(),
            proposals_id,
            VoteKind::Approve
        )
        .is_ok());

        assert!(ProposalsEngine::vote(
            system::RawOrigin::Signed(3).into(),
            proposals_id,
            VoteKind::Reject
        )
        .is_ok());

        assert!(ProposalsEngine::vote(
            system::RawOrigin::Signed(4).into(),
            proposals_id,
            VoteKind::Abstain
        )
        .is_ok());

        run_to_block_and_finalize(2);

        let tally_result = <TallyResults<Test>>::get(proposals_id);

        assert_eq!(
            tally_result,
            TallyResult {
                proposal_id: proposals_id,
                abstentions: 1,
                approvals: 2,
                rejections: 1,
                status: ProposalStatus::Approved,
                finalized_at: 1
            }
        )
    });
}

#[test]
fn rejected_tally_results_and_remove_proposal_id_from_active_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        let origin = system::RawOrigin::Signed(1).into();
        assert!(ProposalsEngine::create_proposal(
            origin,
            parameters,
            text_proposal.proposal_type(),
            text_proposal.encode()
        )
        .is_ok());

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(1).into(),
                proposal_id,
                VoteKind::Reject
            ),
            Ok(())
        );

        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(2).into(),
                proposal_id,
                VoteKind::Reject
            ),
            Ok(())
        );

        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(3).into(),
                proposal_id,
                VoteKind::Abstain
            ),
            Ok(())
        );
        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(4).into(),
                proposal_id,
                VoteKind::Abstain
            ),
            Ok(())
        );

        let mut active_proposals_id = <ActiveProposalIds>::get();

        let mut active_proposals_set = BTreeSet::new();
        active_proposals_set.insert(proposal_id);
        assert_eq!(active_proposals_id, active_proposals_set);

        run_to_block_and_finalize(2);

        let tally_result = <TallyResults<Test>>::get(proposal_id);

        assert_eq!(
            tally_result,
            TallyResult {
                proposal_id,
                abstentions: 2,
                approvals: 0,
                rejections: 2,
                status: ProposalStatus::Rejected,
                finalized_at: 1
            }
        );

        active_proposals_id = <ActiveProposalIds>::get();
        assert_eq!(active_proposals_id, BTreeSet::new());
    });
}
