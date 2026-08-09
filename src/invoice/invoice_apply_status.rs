// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Review outcomes for requests to allocate invoice numbers.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The current outcome of an invoice-number allocation request.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceApplyStatus {
    /// Awaiting the platform's review.
    Submitted,
    /// Approved for invoice-number allocation.
    Approved,
    /// Rejected by the reviewing authority.
    Rejected,
    /// Withdrawn before a final approval decision.
    Cancelled,
}
