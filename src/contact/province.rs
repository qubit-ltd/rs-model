// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Administrative-region model values.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A province in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Province {
    /// Optional persisted identifier.
    pub id: Option<i64>,

    /// Globally unique ASCII province code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Globally unique province name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Basic information for the country.
    #[model(opaque)]
    pub country: Info,

    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,

    /// Optional administrative level.
    pub level: Option<i32>,

    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Whether this is predefined reference data.
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
