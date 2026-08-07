// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! High-level claim workflow groups.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Groups detailed individual claim states into reporting stages.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimStatusGroup {
    /// The claim has not been submitted.
    NotSubmitted,
    /// The claim is awaiting case creation.
    PendingCase,
    /// The case was registered; the misspelling preserves the Java wire value.
    Registed,
    /// The claim did not reach its deductible threshold.
    Unreached,
    /// The claim is under review.
    UnderReview,
    /// The insurer rejected the claim during audit.
    AuditRejection,
    /// The system rejected the claim.
    Rejected,
    /// The claim was completed.
    Completed,
    /// The claim was cancelled; the misspelling preserves the Java wire value.
    Canceld,
}
