// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice issuance states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes whether and how an invoice was issued.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    /// The transaction cannot be invoiced.
    NoInvoice,
    /// No invoice is required.
    NotRequired,
    /// An invoice is required but has not been issued.
    NotPrinted,
    /// The invoice was issued.
    Printed,
    /// The invoice was reissued.
    Reprinted,
    /// The invoice was invalidated or reversed.
    Invalid,
}
