// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medicare item classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies the kind of item submitted to medical insurance.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
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
