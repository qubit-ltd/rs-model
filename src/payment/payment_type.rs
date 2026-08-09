// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Business programs under which a payment is settled.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The business program that determines how a payment is processed.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    /// A standard commercial payment.
    Normal,
    /// A payment processed under the Medicare program.
    Medicare,
}
