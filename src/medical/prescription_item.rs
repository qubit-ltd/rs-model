// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medication lines that make up a prescription.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::medical::Dosage;
use crate::medical::DrugInfo;

/// One prescribed medicine, its dispensed quantity, and its administration
/// instructions.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrescriptionItem {
    /// Typed identifier used when this prescription line is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted identifier of the owning prescription.
    #[model(opaque)]
    pub prescription_id: Id,

    /// Prescribed drug information.
    pub drug: DrugInfo,

    /// Package count or dose quantity.
    pub amount: i32,

    /// Unit for the package count or dose.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,

    /// Administration and dosage instructions.
    pub dosage: Dosage,

    /// Prescriber or dispenser note for this line, absent when none is needed.
    pub comment: Option<String>,
}
