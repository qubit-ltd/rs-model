// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States that describe whether an invoice can be or has been issued.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The invoice obligation and issuance outcome for a business transaction.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    /// The transaction is not eligible for invoicing.
    NoInvoice,
    /// No invoice was requested for the transaction.
    NotRequired,
    /// An invoice is required but issuance has not completed.
    NotPrinted,
    /// The original invoice was issued successfully.
    Printed,
    /// A replacement invoice was issued.
    Reprinted,
    /// The invoice was voided or reversed.
    Invalid,
}
