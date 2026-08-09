// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States that govern the order fulfilment lifecycle.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The current commercial and fulfilment state of an order.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order was not completed before its payment deadline.
    Expired,
    /// The order was cancelled before completion.
    Cancelled,
    /// The buyer submitted the order for seller processing.
    Submitted,
    /// The seller accepted the order.
    Accepted,
    /// The seller declined the order.
    Rejected,
    /// Payment processing failed.
    PaidFail,
    /// Payment completed successfully.
    PaidSuccess,
    /// The goods were dispatched.
    Sent,
    /// The recipient confirmed receipt.
    Received,
    /// The order completed its fulfilment lifecycle.
    Completed,
}
