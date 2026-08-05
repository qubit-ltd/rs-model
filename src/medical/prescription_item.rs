// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prescription line items.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::medical::{
    Dosage,
    DrugInfo,
};

/// A prescribed drug together with quantity and dosage instructions.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct PrescriptionItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted identifier of the owning prescription.
    pub prescription_id: i64,
    /// Prescribed drug information.
    pub drug: DrugInfo,
    /// Package count or dose quantity.
    pub amount: i32,
    /// Unit for the package count or dose.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,
    /// Administration and dosage instructions.
    pub dosage: Dosage,
    /// Optional remark.
    pub comment: Option<String>,
}
