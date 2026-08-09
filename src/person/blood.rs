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

/// ABO blood group recorded for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Blood {
    /// Records ABO group A for clinical matching and transfusion workflows.
    TypeA,
    /// Records ABO group B for clinical matching and transfusion workflows.
    TypeB,
    /// Records ABO group AB for clinical matching and transfusion workflows.
    TypeAb,
    /// Records ABO group O for clinical matching and transfusion workflows.
    TypeO,
    /// The blood group is unknown.
    Unknown,
}
