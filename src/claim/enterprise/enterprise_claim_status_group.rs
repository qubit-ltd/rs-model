// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! High-level enterprise claim workflow groups.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Groups detailed enterprise claim states into reporting stages.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseClaimStatusGroup {
    /// The claim has not been submitted.
    NotSubmitted,
    /// The claim is being registered.
    Register,
    /// The claim is under audit.
    Audit,
    /// The claim was rejected.
    Reject,
    /// The claim was completed.
    Complete,
    /// The claim was cancelled.
    Cancel,
}
