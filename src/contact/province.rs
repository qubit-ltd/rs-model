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
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Country;

/// A province in the administrative hierarchy.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Province {
    /// Platform-assigned identifier of this province reference record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII province code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Globally unique province name.
    #[model(index, unique(ignore_case), text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Basic information for the country.
    #[model(reference(target = Country, target_field = info), index, opaque)]
    pub country: Info,

    /// Optional ASCII postal code.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,

    /// Optional administrative level.
    #[model(index)]
    pub level: Option<i32>,

    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional explanatory text for the province reference record.
    pub description: Option<String>,

    /// Whether this is predefined reference data.
    #[model(index)]
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
