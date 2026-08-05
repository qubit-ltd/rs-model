// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim persistence states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes whether imported enterprise claim data was saved.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaveStatus {
    /// The data has not been saved.
    NotSaved,
    /// The data was saved.
    Saved,
}
