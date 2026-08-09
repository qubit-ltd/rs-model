// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Terminal and in-progress states for settlement transactions.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The processing state of a purchase or refund transaction.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    /// The transaction passed its expiry deadline.
    Expired,
    /// The transaction was submitted for processing.
    Submitted,
    /// Processing ended in failure.
    Fail,
    /// Processing ended successfully.
    Success,
    /// The transaction was cancelled before completion.
    Cancelled,
}
