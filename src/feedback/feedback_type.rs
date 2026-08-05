// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback classification vocabularies.

#[allow(unused_imports)]
use super::{
    FeedbackAction,
    FeedbackRating,
    FeedbackStatus,
};

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// The user-facing category of a feedback record.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackType {
    /// A complaint.
    Complaint,
    /// A report of a problem or violation.
    Report,
    /// A suggestion for improvement.
    Suggestion,
}
