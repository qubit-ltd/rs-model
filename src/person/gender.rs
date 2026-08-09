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

/// Gender recorded for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    /// Gender is unknown.
    Unknown,
    /// Records a male gender value supplied by the person or source system.
    Male,
    /// Records a female gender value supplied by the person or source system.
    Female,
    /// Gender was intentionally not specified.
    Unspecified,
}
