// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Telephone number value objects.

use serde::Deserialize;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A telephone number split into its source-domain country, city, and local
/// parts.
///
/// The three components preserve the Java model's representation while the
/// `textual` capability permits mobile-format metadata on fields that use it.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq, Eq)]
#[redact(debug, serde)]
#[model(textual)]
pub struct Phone {
    /// Optional international dialing area without the `+` prefix.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub country_area: Option<String>,

    /// Optional city or regional dialing area.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub city_area: Option<String>,

    /// Required local subscriber number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub number: String,
}

impl From<&str> for Phone {
    fn from(number: &str) -> Self {
        Self {
            number: number.to_owned(),
            ..Self::default()
        }
    }
}

impl From<String> for Phone {
    fn from(number: String) -> Self {
        Self {
            number,
            ..Self::default()
        }
    }
}

impl Display for Phone {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if let Some(country_area) = &self.country_area {
            write!(formatter, "+{country_area}-")?;
        }
        if let Some(city_area) = &self.city_area {
            write!(formatter, "{city_area}-")?;
        }
        formatter.write_str(&self.number)
    }
}

impl Normalizable for Phone {
    fn normalize(&mut self) {
        self.country_area.normalize();
        self.city_area.normalize();
        self.number.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.number.is_empty()
    }
}
