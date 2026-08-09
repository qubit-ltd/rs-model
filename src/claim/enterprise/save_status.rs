// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persistence outcomes for enterprise-claim data received from an external source.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Records whether an imported enterprise-claim record has been persisted.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SaveStatus {
    /// The data has not been saved.
    NotSaved,
    /// The data was saved.
    Saved,
}
