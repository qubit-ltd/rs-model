// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Access-token model values.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Represents a token together with its issuance and lifetime metadata.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Token {
    /// Current token value.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub value: String,

    /// UTC timestamp at which the token was issued.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional maximum lifetime in whole seconds.
    pub max_age: Option<i64>,

    /// Optional value of the immediately preceding token.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub previous_value: Option<String>,
}
