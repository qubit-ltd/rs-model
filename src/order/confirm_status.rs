// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Confirmation states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes whether a confirmation request expired or was accepted.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfirmStatus {
    /// The confirmation window expired.
    Expired,
    /// The request was accepted.
    Accepted,
}
