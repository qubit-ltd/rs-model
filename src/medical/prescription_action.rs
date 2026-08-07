// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prescription workflow actions.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies an action applied to a prescription workflow.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrescriptionAction {
    /// Creates the prescription.
    Create,
    /// Accepts the prescription during the hospital pharmacist audit.
    AuditorAccept,
    /// Rejects the prescription during the hospital pharmacist audit.
    AuditorReject,
    /// Accepts the prescription during the third-party inspection.
    InspectorAccept,
    /// Rejects the prescription during the third-party inspection.
    InspectorReject,
    /// Records the patient's acceptance.
    PatientAccept,
    /// Records the patient's rejection.
    PatientReject,
    /// Transfers the prescription to a pharmacy or hospital.
    Transfer,
    /// Records that a pharmacist prepared the medication.
    Prepare,
    /// Accepts the prepared medication during pharmacist review.
    ReviewerAccept,
    /// Rejects the prepared medication during pharmacist review.
    ReviewerReject,
    /// Dispatches the prescribed medication.
    Dispatch,
    /// Records the patient's receipt of the medication.
    Receive,
    /// Cancels the prescription.
    Cancel,
}
