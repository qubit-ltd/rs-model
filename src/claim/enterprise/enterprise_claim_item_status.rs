// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim-item states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes calculation of an enterprise claim item.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseClaimItemStatus {
    /// The item was created and awaits calculation.
    Created,
    /// The item is invalid.
    Disabled,
    /// Calculation completed.
    Completed,
}
