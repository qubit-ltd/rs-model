// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statutory forms of invoices.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The statutory invoice form requested or issued.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceType {
    /// A standard commercial invoice.
    Normal,
    /// A special invoice for value-added-tax purposes.
    ValueAdded,
}
