// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Actions that move feedback through its processing lifecycle.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// An operation requested by either the submitter or an administrator.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackAction {
    /// The submitter withdraws feedback before administrative processing.
    UserWithdraw,
    /// The submitter accepts a resolution or rejection and closes the case.
    UserApprove,
    /// The submitter disputes a resolution or rejection.
    UserDisapprove,
    /// An administrator begins reviewing the feedback.
    AdminReview,
    /// An administrator accepts the feedback for handling.
    AdminAccept,
    /// An administrator rejects the feedback during review.
    AdminReject,
    /// An administrator records that processing is complete.
    AdminResolve,
    /// An administrator reopens a case after the submitter disapproves its outcome.
    AdminReopen,
}
