// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Disease reference records.

use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// A coded disease definition used by diagnoses and medical catalogs.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Disease {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
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
