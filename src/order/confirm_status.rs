// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Confirmation states.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Describes whether a confirmation request expired or was accepted.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfirmStatus {
    /// The confirmation window expired.
    Expired,
    /// The request was accepted.
    Accepted,
}
