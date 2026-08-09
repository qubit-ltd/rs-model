// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Religious affiliation reported for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Religion {
    /// No religious affiliation.
    None,
    /// Confucianism.
    Confucianism,
    /// Taoism.
    Taoism,
    /// Buddhism.
    Buddhism,
    /// Shinto.
    Shinto,
    /// Christianity.
    Christianity,
    /// Judaism.
    Judaism,
    /// Islam.
    Islam,
}
