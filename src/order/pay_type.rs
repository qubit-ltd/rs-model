// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order payment classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes the special payment program used by an order.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayType {
    /// Payment through Medicare.
    Medicare,
}
