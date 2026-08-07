// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Referral order record states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes the payment outcome tracked for a referred order.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefererOrderRecordStatus {
    /// The order was submitted.
    Submitted,
    /// Payment failed.
    PaidFail,
    /// Payment succeeded.
    PaidSuccess,
    /// The order was refunded.
    Refund,
}
