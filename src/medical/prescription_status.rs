// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prescription workflow states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes the current state of a prescription workflow.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrescriptionStatus {
    /// The prescription was created.
    Created,
    /// The hospital pharmacist accepted the prescription.
    AuditorAccepted,
    /// The hospital pharmacist rejected the prescription.
    AuditorRejected,
    /// The third-party pharmacist accepted the prescription.
    InspectorAccepted,
    /// The third-party pharmacist rejected the prescription.
    InspectorRejected,
    /// The patient accepted the prescription.
    PatientAccepted,
    /// The patient rejected the prescription.
    PatientRejected,
    /// The prescription was transferred to a pharmacy or hospital.
    Transferred,
    /// The prescribed medication was prepared.
    Prepared,
    /// The reviewing pharmacist accepted the prepared medication.
    ReviewerAccepted,
    /// The reviewing pharmacist rejected the prepared medication.
    ReviewerRejected,
    /// The medication was dispatched.
    Dispatched,
    /// The patient received the medication.
    Received,
    /// The prescription expired.
    Expired,
    /// The prescription was cancelled.
    Cancelled,
}
