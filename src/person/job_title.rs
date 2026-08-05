// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

#[allow(unused_imports)]
use super::{
    Blood,
    Education,
    Ethnic,
    Gender,
    Incoming,
    Industry,
    Marriage,
    Person,
    PersonIdentity,
    Politics,
    Religion,
    SexOrientation,
    SocialNetwork,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Source-domain JobTitle classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobTitle {
    /// Source variant `FREELANCE`.
    Freelance,
    /// Source variant `EMPLOYEE`.
    Employee,
    /// Source variant `JUNIOR_TITLE`.
    JuniorTitle,
    /// Source variant `MIDDLE_TITLE`.
    MiddleTitle,
    /// Source variant `SENIOR_TITLE`.
    SeniorTitle,
    /// Source variant `JUNIOR_MANAGER`.
    JuniorManager,
    /// Source variant `MIDDLE_MANAGER`.
    MiddleManager,
    /// Source variant `SENIOR_MANAGER`.
    SeniorManager,
    /// Source variant `OWNER`.
    Owner,
}
