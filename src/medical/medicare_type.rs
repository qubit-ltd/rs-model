// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical-insurance classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies a patient's medical-insurance program.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicareType {
    /// Urban employee basic medical insurance.
    Employee,
    /// Urban resident basic medical insurance.
    Resident,
    /// New rural cooperative medical insurance.
    NewRuralCooperative,
    /// Another medical-insurance classification.
    Other,
}
