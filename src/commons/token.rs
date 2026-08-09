// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Access-token model values.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// An issued bearer token together with the metadata needed to assess its lifetime.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Token {
    /// Secret bearer value presented by the holder for authentication.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub value: String,

    /// UTC issuance time, stored with second precision.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Maximum lifetime in whole seconds, or `None` when no lifetime limit is recorded.
    pub max_age: Option<i64>,

    /// Immediately preceding bearer value, or `None` when this token did not rotate another.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub previous_value: Option<String>,
}
