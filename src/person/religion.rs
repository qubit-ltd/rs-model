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


/// Source-domain Religion classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Religion {
    /// Source variant `NONE`.
    None,
    /// Source variant `CONFUCIANISM`.
    Confucianism,
    /// Source variant `TAOISM`.
    Taoism,
    /// Source variant `BUDDHISM`.
    Buddhism,
    /// Source variant `SHINTO`.
    Shinto,
    /// Source variant `CHRISTIANITY`.
    Christianity,
    /// Source variant `JUDAISM`.
    Judaism,
    /// Source variant `ISLAM`.
    Islam,
}
