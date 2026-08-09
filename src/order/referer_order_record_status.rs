// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment outcomes recorded for referral attribution.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The order outcome used to determine referral attribution.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefererOrderRecordStatus {
    /// The referred order was submitted.
    Submitted,
    /// The associated payment failed.
    PaidFail,
    /// The associated payment succeeded.
    PaidSuccess,
    /// The order was refunded after payment.
    Refund,
}
