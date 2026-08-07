// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! The complete legal feedback-state transition table.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::feedback::FeedbackAction;
use crate::feedback::FeedbackStatus;

/// Evaluates the fixed feedback lifecycle defined by the Java source model.
#[derive(Model, Redact, Clone, Copy, Default)]
#[redact(debug, display, serde)]
pub struct FeedbackProcessingRule;

impl FeedbackProcessingRule {
    /// Returns the lifecycle's sole initial state.
    #[must_use]
    pub const fn initial_state() -> FeedbackStatus {
        FeedbackStatus::Submitted
    }

    /// Reports whether `status` is one of the lifecycle's terminal states.
    #[must_use]
    pub const fn is_final_state(status: FeedbackStatus) -> bool {
        matches!(status, FeedbackStatus::Closed | FeedbackStatus::Withdrawn)
    }

    /// Returns the state reached by applying `action` to `status`.
    ///
    /// `None` means that the Java transition table does not permit the pair.
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
