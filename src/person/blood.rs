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


/// Source-domain Blood classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Blood {
    /// Source variant `TYPE_A`.
    TypeA,
    /// Source variant `TYPE_B`.
    TypeB,
    /// Source variant `TYPE_AB`.
    TypeAb,
    /// Source variant `TYPE_O`.
    TypeO,
    /// Source variant `UNKNOWN`.
    Unknown,
}
