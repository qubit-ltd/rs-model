// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! The fixed state-transition rules for feedback processing.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::feedback::FeedbackAction;
use crate::feedback::FeedbackStatus;

/// Evaluates the feedback lifecycle shared with the source-domain model.
#[derive(Model, Redact, Clone, Copy, Default)]
#[redact(debug, display, serde)]
pub struct FeedbackProcessingRule;

impl FeedbackProcessingRule {
    /// Returns the state assigned to every newly submitted feedback record.
    #[must_use]
    pub const fn initial_state() -> FeedbackStatus {
        FeedbackStatus::Submitted
    }

    /// Returns whether `status` has no legal follow-up action.
    #[must_use]
    pub const fn is_final_state(status: FeedbackStatus) -> bool {
        matches!(status, FeedbackStatus::Closed | FeedbackStatus::Withdrawn)
    }

    /// Applies `action` to `status` according to the fixed lifecycle table.
    ///
    /// Returns `None` when the action is not permitted from the supplied state.
    #[must_use]
    pub const fn next(status: FeedbackStatus, action: FeedbackAction) -> Option<FeedbackStatus> {
        match (status, action) {
            (FeedbackStatus::Submitted, FeedbackAction::UserWithdraw) => {
                Some(FeedbackStatus::Withdrawn)
            }
            (FeedbackStatus::Submitted, FeedbackAction::AdminReview)
            | (FeedbackStatus::Reopened, FeedbackAction::AdminReview) => {
                Some(FeedbackStatus::UnderReview)
            }
            (FeedbackStatus::UnderReview, FeedbackAction::AdminAccept) => {
                Some(FeedbackStatus::Processing)
            }
            (FeedbackStatus::UnderReview, FeedbackAction::AdminReject) => {
                Some(FeedbackStatus::Rejected)
            }
            (FeedbackStatus::Processing, FeedbackAction::AdminResolve) => {
                Some(FeedbackStatus::Resolved)
            }
            (FeedbackStatus::Rejected | FeedbackStatus::Resolved, FeedbackAction::UserApprove) => {
                Some(FeedbackStatus::Closed)
            }
            (
                FeedbackStatus::Rejected | FeedbackStatus::Resolved,
                FeedbackAction::UserDisapprove,
            ) => Some(FeedbackStatus::Disapproved),
            (FeedbackStatus::Disapproved, FeedbackAction::AdminReopen) => {
                Some(FeedbackStatus::Reopened)
            }
            _ => None,
        }
    }
}
