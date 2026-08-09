// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Customer interaction modes supported by payment providers.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The client or merchant interaction through which a payment is initiated.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMode {
    /// A native mobile application.
    App,
    /// A provider-hosted lightweight application.
    LiteApp,
    /// A mobile web page.
    Wap,
    /// A desktop browser page.
    Web,
    /// A JavaScript API integration.
    Jsapi,
    /// The customer scans a merchant-generated QR code.
    ActiveQr,
    /// The merchant scans a customer-presented QR code.
    PassiveQr,
    /// A payment completed outside an online provider flow.
    Offline,
    /// An interaction mode not recognized by the source system.
    Unknown,
}
