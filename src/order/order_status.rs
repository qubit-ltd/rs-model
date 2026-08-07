// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order lifecycle states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes an order's lifecycle state.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order expired.
    Expired,
    /// The order was cancelled.
    Cancelled,
    /// The order was submitted.
    Submitted,
    /// The seller accepted the order.
    Accepted,
    /// The seller rejected the order.
    Rejected,
    /// Payment failed.
    PaidFail,
    /// Payment succeeded.
    PaidSuccess,
    /// The order was sent.
    Sent,
    /// The order was received.
    Received,
    /// The order completed.
    Completed,
}
