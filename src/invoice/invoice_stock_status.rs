// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Inventory states for allocated invoice-number ranges.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Whether an allocated invoice-number segment is available for use.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStockStatus {
    /// Allocation has not yet been entered into inventory.
    Unstocked,
    /// The number range is stocked and available for issuance.
    Stocked,
    /// The allocated range was cancelled and must not be used.
    Cancelled,
}
