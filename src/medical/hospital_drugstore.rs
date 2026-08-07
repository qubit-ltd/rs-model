// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Hospital-to-drugstore relationships.

use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A relationship between a hospital and one of its dispensing drugstores.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct HospitalDrugstore {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

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
