// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::commons::App;
use qubit_model::feedback::Feedback;
use qubit_model::feedback::FeedbackAction;
use qubit_model::feedback::FeedbackProcessingRule;
use qubit_model::feedback::FeedbackRating;
use qubit_model::feedback::FeedbackStatus;
use qubit_model::feedback::FeedbackTrack;
use qubit_model::feedback::FeedbackType;
use qubit_model::person::User;
use qubit_model::upload::Attachment;
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
        serde_json::to_string(&FeedbackType::Complaint).expect("feedback type should serialize"),
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
        FeedbackProcessingRule::next(FeedbackStatus::Submitted, FeedbackAction::AdminReview,),
        Some(FeedbackStatus::UnderReview)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::UnderReview, FeedbackAction::AdminAccept,),
        Some(FeedbackStatus::Processing)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::UnderReview, FeedbackAction::AdminReject,),
        Some(FeedbackStatus::Rejected)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Processing, FeedbackAction::AdminResolve,),
        Some(FeedbackStatus::Resolved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Resolved, FeedbackAction::UserApprove,),
        Some(FeedbackStatus::Closed)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Rejected, FeedbackAction::UserApprove,),
        Some(FeedbackStatus::Closed)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Rejected, FeedbackAction::UserDisapprove,),
        Some(FeedbackStatus::Disapproved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Resolved, FeedbackAction::UserDisapprove,),
        Some(FeedbackStatus::Disapproved)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Disapproved, FeedbackAction::AdminReopen,),
        Some(FeedbackStatus::Reopened)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Reopened, FeedbackAction::AdminReview,),
        Some(FeedbackStatus::UnderReview)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Submitted, FeedbackAction::UserWithdraw,),
        Some(FeedbackStatus::Withdrawn)
    );
    assert_eq!(
        FeedbackProcessingRule::next(FeedbackStatus::Processing, FeedbackAction::UserDisapprove,),
        None
    );
}

/// Verifies feedback records and tracks preserve their authoritative indexes
/// and direct-reference metadata.
#[test]
fn test_feedback_metadata_preserves_indexes_and_references() {
    let feedback = metadata_of::<Feedback>();
    for field in [
        "type",
        "category",
        "status",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(
            feedback.indexes().any(|index| index.contains(field)),
            "feedback is missing an index for {field}"
        );
    }
    let app = feedback
        .field("app")
        .expect("feedback app field")
        .reference()
        .expect("feedback app reference");
    assert_eq!(
        app.target().identity().type_name(),
        core::any::type_name::<App>()
    );
    assert_eq!(app.target_field().segments(), ["info"]);
    assert!(app.must_exist());
    let submitter = feedback
        .field("submitter")
        .expect("feedback submitter field")
        .reference()
        .expect("feedback submitter reference");
    assert_eq!(
        submitter.target().identity().type_name(),
        core::any::type_name::<User>()
    );
    assert_eq!(submitter.target_field().segments(), ["info"]);
    assert!(submitter.must_exist());
    for field in ["voice", "attachments"] {
        let reference = feedback
            .field(field)
            .expect("feedback attachment field")
            .reference()
            .expect("feedback attachment reference");
        assert_eq!(
            reference.target().identity().type_name(),
            core::any::type_name::<Attachment>()
        );
        assert_eq!(reference.target_field().segments(), ["id"]);
        assert!(!reference.must_exist());
    }

    let track = metadata_of::<FeedbackTrack>();
    assert!(track.primary_key().is_some());
    let feedback_id = track
        .field("feedback_id")
        .expect("feedback track feedback ID field")
        .reference()
        .expect("feedback track feedback reference");
    assert_eq!(
        feedback_id.target().identity().type_name(),
        core::any::type_name::<Feedback>()
    );
    assert_eq!(feedback_id.target_field().segments(), ["id"]);
    assert!(feedback_id.must_exist());
    for field in [
        "action",
        "status_before",
        "status_after",
        "rating",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(
            track.indexes().any(|index| index.contains(field)),
            "feedback track is missing an index for {field}"
        );
    }
    let track_submitter = track
        .field("submitter")
        .expect("feedback track submitter field")
        .reference()
        .expect("feedback track submitter reference");
    assert_eq!(
        track_submitter.target().identity().type_name(),
        core::any::type_name::<User>()
    );
    assert_eq!(track_submitter.target_field().segments(), ["info"]);
    assert!(track_submitter.must_exist());
    for field in ["record", "attachments"] {
        let reference = track
            .field(field)
            .expect("feedback track attachment field")
            .reference()
            .expect("feedback track attachment reference");
        assert_eq!(
            reference.target().identity().type_name(),
            core::any::type_name::<Attachment>()
        );
        assert_eq!(reference.target_field().segments(), ["id"]);
        assert!(!reference.must_exist());
    }
}
