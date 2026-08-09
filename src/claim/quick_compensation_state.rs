// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States of the external quick-compensation lookup.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Reports whether quick-compensation data is pending, available, or unavailable.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuickCompensationState {
    /// Data retrieval is in progress.
    Fetching,
    /// Data retrieval succeeded.
    Success,
    /// Data retrieval failed.
    Failed,
}
