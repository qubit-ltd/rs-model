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

/// Marital status recorded for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Marriage {
    /// Never married.
    Unmarried,
    /// Married.
    Married,
    /// In a first marriage.
    MarriedFirstTime,
    /// Remarried.
    MarriedAgain,
    /// Marriage restored after divorce.
    MarriedRestored,
    /// Widowed.
    Widowed,
    /// Divorced.
    Divorced,
    /// Legally or informally separated.
    Separated,
    /// Marital status was not provided.
    Unprovided,
}
