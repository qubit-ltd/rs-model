// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback classification vocabularies.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// An operation applied to a feedback record.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
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
