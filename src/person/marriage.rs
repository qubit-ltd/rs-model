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

/// Source-domain Marriage classification.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Marriage {
    /// Source variant `UNMARRIED`.
    Unmarried,
    /// Source variant `MARRIED`.
    Married,
    /// Source variant `MARRIED_FIRST_TIME`.
    MarriedFirstTime,
    /// Source variant `MARRIED_AGAIN`.
    MarriedAgain,
    /// Source variant `MARRIED_RESTORED`.
    MarriedRestored,
    /// Source variant `WIDOWED`.
    Widowed,
    /// Source variant `DIVORCED`.
    Divorced,
    /// Source variant `SEPARATED`.
    Separated,
    /// Source variant `UNPROVIDED`.
    Unprovided,
}
