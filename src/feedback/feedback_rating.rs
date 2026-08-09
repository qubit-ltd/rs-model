// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Satisfaction ratings supplied for feedback outcomes.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// The submitter's assessment of a feedback outcome.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackRating {
    /// The submitter considers the outcome satisfactory.
    Satisfied,
    /// The submitter gives neither positive nor negative approval.
    Neutral,
    /// The submitter considers the outcome unsatisfactory.
    Dissatisfied,
}
