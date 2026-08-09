// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Classifications for the medical-insurance scheme covering a patient.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the basic medical-insurance program used for settlement.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
