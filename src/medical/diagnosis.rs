// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Ranked diagnoses assigned to clinical records.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::Entity;

/// A prioritized Western-medicine or traditional-Chinese diagnosis for one
/// clinical record.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Diagnosis {
    /// Typed identifier used when this diagnosis is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Entity classification of the owning record.
    pub owner_type: Entity,

    /// Persisted identifier of the owning record.
    #[model(opaque)]
    pub owner_id: Id,

    /// Western-medicine disease entry; absent for a diagnosis recorded only in
    /// the traditional-Chinese classification.
    #[model(opaque)]
    pub disease: Option<Info>,

    /// Traditional-Chinese-medicine disease entry; absent for a Western-only
    /// diagnosis.
    #[model(opaque)]
    pub tcm_disease: Option<Info>,

    /// Symptom or traditional syndrome qualifier, absent when not documented.
    pub syndrome: Option<String>,

    /// Free-text clinical diagnosis detail, absent when the coded entry suffices.
    pub description: Option<String>,

    /// Source or clinician remark, absent when no supplementary note was given.
    pub comment: Option<String>,

    /// Diagnosis priority, starting at zero for the primary diagnosis.
    pub priority: i32,
}
