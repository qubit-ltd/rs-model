// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Classifications for the kind of medical encounter being recorded.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the care event represented by a medical record or settlement.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicalType {
    /// Registration encounter.
    Registration,
    /// General clinic encounter.
    Clinic,
    /// Special outpatient clinic encounter.
    SpecificClinic,
    /// Emergency clinic encounter.
    EmergentClinic,
    /// Hospital stay.
    Hospitalization,
    /// Hospital admission.
    Admission,
    /// Hospital discharge.
    Discharge,
    /// Medical examination.
    Examination,
    /// Internet-hospital encounter.
    Internet,
    /// Unknown medical encounter.
    Unknown,
}
