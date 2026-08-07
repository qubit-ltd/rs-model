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

/// A user's satisfaction rating for a feedback outcome.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackRating {
    /// The user is satisfied.
    Satisfied,
    /// The user is neither satisfied nor dissatisfied.
    Neutral,
    /// The user is dissatisfied.
    Dissatisfied,
}
