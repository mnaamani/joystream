mod mock;

use crate::*;
use mock::*;

use codec::Encode;
use rstd::collections::btree_set::BTreeSet;
use runtime_primitives::traits::{OnFinalize, OnInitialize};
use srml_support::{dispatch, StorageMap, StorageValue};
use system::RawOrigin;

struct TextProposalFixture {
    parameters: ProposalParameters<u64>,
    origin: RawOrigin<u64>,
}

impl Default for TextProposalFixture {
    fn default() -> Self {
        TextProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
            },
            origin: RawOrigin::Signed(1),
        }
    }
}

impl TextProposalFixture {
    fn with_parameters(self, parameters: ProposalParameters<u64>) -> Self {
        TextProposalFixture {
            parameters,
            origin: self.origin,
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        TextProposalFixture {
            parameters: self.parameters,
            origin,
        }
    }

    fn call_and_assert(self, result: dispatch::Result) {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        assert_eq!(
            ProposalsEngine::create_proposal(
                self.origin.into(),
                self.parameters,
                text_proposal.proposal_type(),
                text_proposal.encode()
            ),
            result
        );
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: u64,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_account_id: 0,
        }
    }
    fn vote_and_assert(&mut self, vote_kind: VoteKind) {
        self.current_account_id += 1;

        assert_eq!(
            ProposalsEngine::vote(
                system::RawOrigin::Signed(self.current_account_id).into(),
                self.proposal_id,
                vote_kind
            ),
            Ok(())
        );
    }
}

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
        let text_proposal = TextProposalFixture::default();

        text_proposal.call_and_assert(Ok(()));
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
        let text_proposal = TextProposalFixture::default().with_origin(RawOrigin::None);

        text_proposal.call_and_assert(Err("Invalid origin"));
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.call_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert(VoteKind::Approve);
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
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        let text_proposal = TextProposalFixture::default().with_parameters(parameters);
        text_proposal.call_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposals_id);
        vote_generator.vote_and_assert(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Approve);

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
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 49,
        };

        let text_proposal = TextProposalFixture::default().with_parameters(parameters);
        text_proposal.call_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposals_id);
        vote_generator.vote_and_assert(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Approve);
        vote_generator.vote_and_assert(VoteKind::Reject);
        vote_generator.vote_and_assert(VoteKind::Abstain);

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
        let text_proposal = TextProposalFixture::default();
        text_proposal.call_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert(VoteKind::Reject);
        vote_generator.vote_and_assert(VoteKind::Reject);
        vote_generator.vote_and_assert(VoteKind::Abstain);
        vote_generator.vote_and_assert(VoteKind::Abstain);

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
