// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Claim-invoice ingestion states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes how an invoice was handled during quick-compensation ingestion.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimInvoiceStatus {
    /// The invoice was saved.
    Saved,
    /// The quick-compensation amount exceeded the invoice amount.
    IgnoredGt,
    /// The quick-compensation amount was below the invoice amount.
    IgnoredLt,
    /// No matching invoice data was available.
    IgnoredNone,
    /// Medical insurance prohibited the invoiced expense.
    IgnoredMedicareProhibited,
    /// The invoice was already saved.
    IgnoredRepeat,
}
