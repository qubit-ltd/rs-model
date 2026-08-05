// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-stock states.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Describes the stock state of an invoice-number segment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStockStatus {
    /// The invoice numbers have not entered stock.
    Unstocked,
    /// The invoice numbers entered stock.
    Stocked,
    /// The stock entry was cancelled.
    Cancelled,
}
