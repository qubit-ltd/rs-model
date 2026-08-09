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

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A country in the administrative hierarchy.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(
    unique(name = "country_code", fields(code), ignore_case(code)),
    unique(name = "country_name", fields(name), ignore_case(name))
)]
pub struct Country {
    /// Platform-assigned identifier of this country reference record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII country code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Globally unique country name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Telephone area code.
    #[model(index, text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub phone_area: String,

    /// Optional ASCII postal code.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,

    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional explanatory text for the country reference record.
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
