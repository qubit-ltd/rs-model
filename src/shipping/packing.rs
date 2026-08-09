// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Protection levels required when a shipment is packed.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The packing protection required for safe shipment handling.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Packing {
    /// Standard packing suitable for ordinary goods.
    Normal,
    /// Reinforced packing for additional protection.
    Reinforcement,
    /// A protective wooden-frame package.
    WoodenFrame,
}
