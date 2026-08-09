// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Reference records for Western and traditional-Chinese diseases.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

/// A coded disease reference used to classify diagnoses and medical records.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Disease {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique institutional or ICD-10 code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Disease name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional national-standard disease category.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Optional description.
    pub description: Option<String>,

    /// Optional detail URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional remark.
    pub comment: Option<String>,

    /// Whether this disease is predefined reference data.
    pub predefined: bool,

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
