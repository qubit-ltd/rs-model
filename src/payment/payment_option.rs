// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Choices for allocating responsibility for an eligible charge.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Selects the party that bears an eligible transaction amount.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentOption {
    /// The customer pays the amount personally.
    PaidBySelf,
    /// The Medicare program pays the amount.
    PaidByMedicare,
}
