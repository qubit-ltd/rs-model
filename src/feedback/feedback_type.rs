// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User-facing categories for feedback submissions.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Category selected by the submitter to express the purpose of feedback.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeedbackType {
    /// Dissatisfaction with a product, service, or experience.
    Complaint,
    /// Report of a defect, incident, or policy violation.
    Report,
    /// Proposal to improve a product, service, or process.
    Suggestion,
}
