// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medicare item classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the kind of item submitted to medical insurance.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicareItemType {
    /// Pharmaceutical drug.
    Drug,
    /// Medical procedure or item.
    Item,
    /// Medical material.
    Material,
    /// Medical service or facility.
    Service,
}
