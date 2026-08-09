// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Managed associations between hospitals and their dispensing drugstores.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// A hospital's authorized relationship with a drugstore that dispenses its
/// prescriptions.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HospitalDrugstore {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Hospital information.
    #[model(opaque)]
    pub hospital: Info,

    /// Drugstore information.
    #[model(opaque)]
    pub drugstore: Info,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
