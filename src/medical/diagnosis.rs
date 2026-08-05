// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical diagnoses.

use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::Entity;

/// A ranked western or traditional-Chinese diagnosis owned by a medical record.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Diagnosis {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Entity classification of the owning record.
    pub owner_type: Entity,
    /// Persisted identifier of the owning record.
    pub owner_id: i64,
    /// Optional western-medicine disease information.
    #[model(opaque)]
    pub disease: Option<Info>,
    /// Optional traditional-Chinese-medicine disease information.
    #[model(opaque)]
    pub tcm_disease: Option<Info>,
    /// Optional symptom or traditional syndrome.
    pub syndrome: Option<String>,
    /// Optional diagnosis description.
    pub description: Option<String>,
    /// Optional remark.
    pub comment: Option<String>,
    /// Diagnosis priority, starting at zero for the primary diagnosis.
    pub priority: i32,
}
