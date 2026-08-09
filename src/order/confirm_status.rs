// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Outcomes of a time-limited confirmation request.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The terminal outcome of a confirmation request.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfirmStatus {
    /// The response window closed before acceptance.
    Expired,
    /// The requested confirmation was accepted in time.
    Accepted,
    /// The requested confirmation was rejected in time.
    Rejected,
}
