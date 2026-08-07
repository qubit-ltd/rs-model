// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback classification vocabularies.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// The processing state of a feedback record.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
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
