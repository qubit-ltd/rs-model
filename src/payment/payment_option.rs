// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment responsibility options.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Selects who pays an eligible charge.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentOption {
    /// The customer pays the charge.
    PaidBySelf,
    /// Medicare pays the charge.
    PaidByMedicare,
}
