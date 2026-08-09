// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Commercial directions represented by settlement transactions.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Whether a transaction collects a purchase payment or sends a refund.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// A transaction that collects payment for a purchase.
    Buy,
    /// A transaction that returns money for a prior purchase.
    Refund,
}
