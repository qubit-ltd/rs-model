// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the legal form of an invoice.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceType {
    /// Ordinary invoice.
    Normal,
    /// Value-added-tax special invoice.
    ValueAdded,
}
