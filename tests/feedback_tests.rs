// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::feedback::{
    Feedback,
    FeedbackAction,
    FeedbackProcessingRule,
    FeedbackRating,
    FeedbackStatus,
    FeedbackTrack,
    FeedbackType,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_feedback_public_types_expose_model_and_redact_contracts() {
    assert_eq!(metadata_of::<Feedback>().struct_fields().len(), 18);
    assert_eq!(metadata_of::<FeedbackTrack>().struct_fields().len(), 14);
    assert_redact::<Feedback>();
    assert_redact::<FeedbackTrack>();
    assert_redact::<FeedbackAction>();
    assert_redact::<FeedbackProcessingRule>();
    assert_redact::<FeedbackRating>();
    assert_redact::<FeedbackStatus>();
    assert_redact::<FeedbackType>();
}

#[test]
fn test_feedback_enums_preserve_java_wire_names() {
    assert_eq!(
        serde_json::to_string(&FeedbackType::Complaint)
            .expect("feedback type should serialize"),
        "\"COMPLAINT\""
    );
    assert_eq!(
        serde_json::to_string(&FeedbackStatus::UnderReview)
            .expect("feedback status should serialize"),
        "\"UNDER_REVIEW\""
    );
    assert_eq!(
        serde_json::to_string(&FeedbackAction::UserDisapprove)
            .expect("feedback action should serialize"),
        "\"USER_DISAPPROVE\""
    );
    assert_eq!(
        serde_json::to_string(&FeedbackRating::Dissatisfied)
            .expect("feedback rating should serialize"),
        "\"DISSATISFIED\""
    );
}

#[test]
fn test_feedback_processing_rule_preserves_complete_lifecycle() {
    assert_eq!(
        FeedbackProcessingRule::initial_state(),
        FeedbackStatus::Submitted
    );
    assert!(FeedbackProcessingRule::is_final_state(
        FeedbackStatus::Closed
    ));
    assert!(FeedbackProcessingRule::is_final_state(
        FeedbackStatus::Withdrawn
    ));
    assert!(!FeedbackProcessingRule::is_final_state(
        FeedbackStatus::Processing
    ));
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Submitted,
            FeedbackAction::AdminReview,
        ),
        Some(FeedbackStatus::UnderReview)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::UnderReview,
            FeedbackAction::AdminAccept,
        ),
        Some(FeedbackStatus::Processing)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::UnderReview,
            FeedbackAction::AdminReject,
        ),
        Some(FeedbackStatus::Rejected)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Processing,
            FeedbackAction::AdminResolve,
        ),
        Some(FeedbackStatus::Resolved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Resolved,
            FeedbackAction::UserApprove,
        ),
        Some(FeedbackStatus::Closed)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Rejected,
            FeedbackAction::UserApprove,
        ),
        Some(FeedbackStatus::Closed)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Rejected,
            FeedbackAction::UserDisapprove,
        ),
        Some(FeedbackStatus::Disapproved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Resolved,
            FeedbackAction::UserDisapprove,
        ),
        Some(FeedbackStatus::Disapproved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Disapproved,
            FeedbackAction::AdminReopen,
        ),
        Some(FeedbackStatus::Reopened)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Reopened,
            FeedbackAction::AdminReview,
        ),
        Some(FeedbackStatus::UnderReview)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Submitted,
            FeedbackAction::UserWithdraw,
        ),
        Some(FeedbackStatus::Withdrawn)
    );
    assert_eq!(
        FeedbackProcessingRule::next(
            FeedbackStatus::Processing,
            FeedbackAction::UserDisapprove,
        ),
        None
    );
}
