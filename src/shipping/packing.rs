// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shipping packing requirements.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Describes how a shipment must be packed.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Packing {
    /// Ordinary packing.
    Normal,
    /// Reinforced packing.
    Reinforcement,
    /// A protective wooden frame.
    WoodenFrame,
}
