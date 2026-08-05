// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment responsibility options.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Selects who pays an eligible charge.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentOption {
    /// The customer pays the charge.
    PaidBySelf,
    /// Medicare pays the charge.
    PaidByMedicare,
}
