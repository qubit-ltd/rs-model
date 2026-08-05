// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-application states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes the review state of an invoice application.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceApplyStatus {
    /// The application was submitted for review.
    Submitted,
    /// The application was approved.
    Approved,
    /// The application was rejected.
    Rejected,
    /// The application was cancelled.
    Cancelled,
}
