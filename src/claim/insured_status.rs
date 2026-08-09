// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Post-treatment outcomes reported for the insured person.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes the claimant-reported outcome of treatment for the insured person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuredStatus {
    /// The insured person recovered.
    Recovery,
    /// The insured person remains under treatment.
    UnderTreatment,
    /// The insured person died.
    Death,
    /// Another treatment outcome applies.
    Other,
}
