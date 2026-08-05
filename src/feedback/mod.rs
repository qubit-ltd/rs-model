// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback records, tracking events, and lifecycle rules.

#[allow(clippy::module_inception)]
mod feedback;
mod feedback_processing_rule;
mod feedback_track;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

pub use feedback::Feedback;
pub use feedback_processing_rule::FeedbackProcessingRule;
pub use feedback_track::FeedbackTrack;

/// An operation applied to a feedback record.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackAction {
    /// The user withdraws a submitted record.
    UserWithdraw,
    /// The user accepts a resolved or rejected outcome.
    UserApprove,
    /// The user rejects a resolved or rejected outcome.
    UserDisapprove,
    /// An administrator starts reviewing the record.
    AdminReview,
    /// An administrator accepts the record for processing.
    AdminAccept,
    /// An administrator rejects the record.
    AdminReject,
    /// An administrator marks the record as resolved.
    AdminResolve,
    /// An administrator reopens a disapproved record.
    AdminReopen,
}

/// A user's satisfaction rating for a feedback outcome.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackRating {
    /// The user is satisfied.
    Satisfied,
    /// The user is neither satisfied nor dissatisfied.
    Neutral,
    /// The user is dissatisfied.
    Dissatisfied,
}

/// The processing state of a feedback record.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackStatus {
    /// The feedback has been submitted.
    Submitted,
    /// The feedback is under administrative review.
    UnderReview,
    /// The accepted feedback is being processed.
    Processing,
    /// The feedback was rejected.
    Rejected,
    /// Processing is complete and awaits user confirmation.
    Resolved,
    /// The user rejected the outcome.
    Disapproved,
    /// The user accepted the outcome and the feedback is closed.
    Closed,
    /// A disapproved record has been reopened.
    Reopened,
    /// The feedback was withdrawn.
    Withdrawn,
}

/// The user-facing category of a feedback record.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackType {
    /// A complaint.
    Complaint,
    /// A report of a problem or violation.
    Report,
    /// A suggestion for improvement.
    Suggestion,
}
