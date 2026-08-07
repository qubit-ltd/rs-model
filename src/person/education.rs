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

/// Source-domain Education classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Education {
    /// Source variant `NONE`.
    None,
    /// Source variant `ELEMENTARY_SCHOOL`.
    ElementarySchool,
    /// Source variant `JUNIOR_HIGH_SCHOOL`.
    JuniorHighSchool,
    /// Source variant `SENIOR_HIGH_SCHOOL`.
    SeniorHighSchool,
    /// Source variant `VOCATIONAL_SENIOR_HIGH_SCHOOL`.
    VocationalSeniorHighSchool,
    /// Source variant `SECONDARY_VOCATIONAL_SCHOOL`.
    SecondaryVocationalSchool,
    /// Source variant `COLLEGE`.
    College,
    /// Source variant `BACHELOR`.
    Bachelor,
    /// Source variant `MASTER`.
    Master,
    /// Source variant `DOCTOR`.
    Doctor,
}
