// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States used to track feedback handling from submission to closure.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Current state of a feedback case in the processing workflow.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackStatus {
    /// The case was submitted and awaits review.
    Submitted,
    /// An administrator is assessing the case.
    UnderReview,
    /// The accepted case is actively being handled.
    Processing,
    /// The case was rejected during review and awaits submitter confirmation.
    Rejected,
    /// Handling is complete and awaits the submitter's assessment.
    Resolved,
    /// The submitter disapproved the outcome.
    Disapproved,
    /// The submitter accepted the outcome, so the case is closed.
    Closed,
    /// A disputed case was reopened for a further review cycle.
    Reopened,
    /// The submitter withdrew the case before it completed.
    Withdrawn,
}
