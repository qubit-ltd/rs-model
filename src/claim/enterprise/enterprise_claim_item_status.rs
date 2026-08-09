// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Calculation states for individual enterprise-claim allocation items.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes whether an enterprise claim allocation item awaits calculation,
/// has been disabled, or is complete.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseClaimItemStatus {
    /// The item was created and awaits calculation.
    Created,
    /// The item is invalid.
    Disabled,
    /// Calculation completed.
    Completed,
}
