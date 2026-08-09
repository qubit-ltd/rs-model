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

/// Highest education level reported for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Education {
    /// No formal education.
    None,
    /// Elementary school.
    ElementarySchool,
    /// Junior high school.
    JuniorHighSchool,
    /// Senior high school.
    SeniorHighSchool,
    /// Vocational senior high school.
    VocationalSeniorHighSchool,
    /// Secondary vocational school.
    SecondaryVocationalSchool,
    /// College or associate degree.
    College,
    /// Bachelor's degree.
    Bachelor,
    /// Master's degree.
    Master,
    /// Doctoral degree.
    Doctor,
}
