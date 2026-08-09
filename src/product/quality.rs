// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Condition classes used in product listings.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Whether the listed product is new or has prior use.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Quality {
    /// A product that has not previously been used.
    BrandNew,
    /// A product offered after prior use.
    Used,
}
