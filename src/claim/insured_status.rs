// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Insured-person treatment outcomes.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes the insured person's outcome after treatment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
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
