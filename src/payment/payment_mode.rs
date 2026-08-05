// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment interaction modes.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Describes the client interaction used to complete a payment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMode {
    /// Native application.
    App,
    /// Lightweight application.
    LiteApp,
    /// Mobile web page.
    Wap,
    /// Desktop web page.
    Web,
    /// JavaScript API.
    Jsapi,
    /// Customer scans a generated QR code.
    ActiveQr,
    /// Merchant scans the customer's QR code.
    PassiveQr,
    /// Offline payment.
    Offline,
    /// Unknown interaction mode.
    Unknown,
}
