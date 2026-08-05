// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Transaction lifecycle states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes a settlement transaction's lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    /// The transaction expired.
    Expired,
    /// The transaction was submitted.
    Submitted,
    /// Processing failed.
    Fail,
    /// Processing succeeded.
    Success,
    /// The transaction was cancelled.
    Cancelled,
}
