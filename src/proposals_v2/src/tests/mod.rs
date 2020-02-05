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
    proposal_type: u32,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    body: Vec<u8>,
}

impl Default for TextProposalFixture {
    fn default() -> Self {
        let text_proposal = crate::codex::TextProposalExecutable {
            title: b"title".to_vec(),
            body: b"body".to_vec(),
        };

        TextProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
            },
            origin: RawOrigin::Signed(1),
            proposal_type: text_proposal.proposal_type(),
            proposal_code: text_proposal.encode(),
            title: text_proposal.title,
            body: text_proposal.body,
        }
    }
}

impl TextProposalFixture {
    fn with_title_and_body(self, title: Vec<u8>, body: Vec<u8>) -> Self {
        TextProposalFixture {
            title,
            body,
            ..self
        }
    }

    fn with_parameters(self, parameters: ProposalParameters<u64>) -> Self {
        TextProposalFixture { parameters, ..self }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        TextProposalFixture { origin, ..self }
    }

    fn with_proposal_type_and_code(self, proposal_type: u32, proposal_code: Vec<u8>) -> Self {
        TextProposalFixture {
            proposal_type,
            proposal_code,
            ..self
        }
    }

    fn create_proposal_and_assert(self, result: dispatch::Result) {
        assert_eq!(
            ProposalsEngine::create_proposal(
                self.origin.into(),
                self.parameters,
                self.title,
                self.body,
                self.proposal_type,
                self.proposal_code,
            ),
            result
        );
    }
}

struct CancelProposalFixture {
    origin: RawOrigin<u64>,
    proposal_id: u32,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(1),
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        CancelProposalFixture { origin, ..self }
    }

    fn cancel_and_assert(self, expected_result: dispatch::Result) {
        assert_eq!(
            ProposalsEngine::cancel_proposal(self.origin.into(), self.proposal_id,),
            expected_result
        );
    }
}
struct VetoProposalFixture {
    origin: RawOrigin<u64>,
    proposal_id: u32,
}

impl VetoProposalFixture {
    fn new(proposal_id: u32) -> Self {
        VetoProposalFixture {
            proposal_id,
            origin: RawOrigin::Root,
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        VetoProposalFixture { origin, ..self }
    }

    fn veto_and_assert(self, expected_result: dispatch::Result) {
        assert_eq!(
            ProposalsEngine::veto_proposal(self.origin.into(), self.proposal_id,),
            expected_result
        );
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_account_id: 0,
            auto_increment_voter_id: true,
        }
    }
    fn vote_and_assert_ok(&mut self, vote_kind: VoteKind) {
        assert_eq!(self.vote(vote_kind), Ok(()));
    }

    fn vote_and_assert(&mut self, vote_kind: VoteKind, expected_result: dispatch::Result) {
        assert_eq!(self.vote(vote_kind), expected_result);
    }

    fn vote(&mut self, vote_kind: VoteKind) -> dispatch::Result {
        if self.auto_increment_voter_id {
            self.current_account_id += 1;
        }

        ProposalsEngine::vote(
            system::RawOrigin::Signed(self.current_account_id).into(),
            self.proposal_id,
            vote_kind,
        )
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

        text_proposal.create_proposal_and_assert(Ok(()));
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

        text_proposal.create_proposal_and_assert(Err("Invalid origin"));
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
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
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposals_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(2);

        let proposal = <crate::engine::Proposals<Test>>::get(proposals_id);

        assert_eq!(
            proposal,
            Proposal {
                proposal_type: 1,
                parameters,
                proposer_id: 1,
                created: 1,
                status: ProposalStatus::Executed,
                title: b"title".to_vec(),
                body: b"body".to_vec(),
            }
        )
    });
}

#[test]
fn proposal_execution_failed() {
    initial_test_ext().execute_with(|| {
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };

        let faulty_proposal = FaultyExecutable;

        let text_proposal = TextProposalFixture::default()
            .with_parameters(parameters)
            .with_proposal_type_and_code(faulty_proposal.proposal_type(), faulty_proposal.encode());

        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposals_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(2);

        let proposal = <crate::engine::Proposals<Test>>::get(proposals_id);

        assert_eq!(
            proposal,
            Proposal {
                proposal_type: faulty_proposal.proposal_type(),
                parameters,
                proposer_id: 1,
                created: 1,
                status: ProposalStatus::Failed {
                    error: "Failed".as_bytes().to_vec()
                },
                title: b"title".to_vec(),
                body: b"body".to_vec(),
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
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposals_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposals_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

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
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

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

#[test]
fn create_text_proposal_fails_with_invalid_body_or_title() {
    initial_test_ext().execute_with(|| {
        let mut text_proposal =
            TextProposalFixture::default().with_title_and_body(Vec::new(), b"body".to_vec());
        text_proposal.create_proposal_and_assert(Err("Proposal cannot have an empty title"));

        text_proposal =
            TextProposalFixture::default().with_title_and_body(b"title".to_vec(), Vec::new());
        text_proposal.create_proposal_and_assert(Err("Proposal cannot have an empty body"));

        let too_long_title = vec![0; 200];
        text_proposal =
            TextProposalFixture::default().with_title_and_body(too_long_title, b"body".to_vec());
        text_proposal.create_proposal_and_assert(Err("Title is too long"));

        let too_long_body = vec![0; 11000];
        text_proposal =
            TextProposalFixture::default().with_title_and_body(b"title".to_vec(), too_long_body);
        text_proposal.create_proposal_and_assert(Err("Body is too long"));
    });
}

#[test]
fn vote_fails_with_expired_voting_period() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        run_to_block_and_finalize(6);

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert(
            VoteKind::Approve,
            Err("Voting period is expired for this proposal"),
        );
    });
}

#[test]
fn vote_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        run_to_block_and_finalize(2);

        let mut vote_generator_to_fail = VoteGenerator::new(proposal_id);
        vote_generator_to_fail
            .vote_and_assert(VoteKind::Approve, Err("Proposal is finalized already"));
    });
}

#[test]
fn vote_fails_with_absent_proposal() {
    initial_test_ext().execute_with(|| {
        let mut vote_generator = VoteGenerator::new(2);
        vote_generator.vote_and_assert(VoteKind::Approve, Err("This proposal does not exist"));
    });
}

#[test]
fn vote_fails_on_double_voting() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.auto_increment_voter_id = false;

        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert(
            VoteKind::Approve,
            Err("You have already voted on this proposal"),
        );
    });
}

#[test]
fn cancel_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };
        let text_proposal = TextProposalFixture::default().with_parameters(parameters);
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Ok(()));

        let proposal = <crate::engine::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                proposal_type: 1,
                parameters,
                proposer_id: 1,
                created: 1,
                status: ProposalStatus::Cancelled,
                title: b"title".to_vec(),
                body: b"body".to_vec(),
            }
        )
    });
}

#[test]
fn cancel_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        run_to_block_and_finalize(6);

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Err("Proposal is finalized already"));
    });
}

#[test]
fn cancel_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let cancel_proposal = CancelProposalFixture::new(2);
        cancel_proposal.cancel_and_assert(Err("This proposal does not exist"));
    });
}

#[test]
fn cancel_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let cancel_proposal =
            CancelProposalFixture::new(proposal_id).with_origin(RawOrigin::Signed(2));
        cancel_proposal.cancel_and_assert(Err("You do not own this proposal"));
    });
}

#[test]
fn veto_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 60,
        };
        let text_proposal = TextProposalFixture::default().with_parameters(parameters);
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        let proposal = <crate::engine::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                proposal_type: 1,
                parameters,
                proposer_id: 1,
                created: 1,
                status: ProposalStatus::Vetoed,
                title: b"title".to_vec(),
                body: b"body".to_vec(),
            }
        )
    });
}

#[test]
fn veto_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        run_to_block_and_finalize(6);

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Err("Proposal is finalized already"));
    });
}

#[test]
fn veto_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let veto_proposal = VetoProposalFixture::new(2);
        veto_proposal.veto_and_assert(Err("This proposal does not exist"));
    });
}

#[test]
fn veto_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let text_proposal = TextProposalFixture::default();
        text_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let veto_proposal =
            VetoProposalFixture::new(proposal_id).with_origin(RawOrigin::Signed(2));
        veto_proposal.veto_and_assert(Err("RequireRootOrigin"));
    });
}
