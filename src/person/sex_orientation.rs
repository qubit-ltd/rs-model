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

/// Sexual orientation reported for a person.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SexOrientation {
    /// Records attraction primarily to people of a different gender.
    Heterosexual,
    /// Records attraction primarily to people of the same gender.
    Homosexual,
    /// Records attraction to people of more than one gender.
    Bisexual,
    /// The person chose not to disclose an orientation.
    Secrecy,
}
