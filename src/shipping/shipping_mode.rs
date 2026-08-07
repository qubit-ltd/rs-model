// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Delivery mode classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes how a product is delivered to its recipient.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingMode {
    /// No delivery is required.
    None,
    /// Delivery by an express carrier.
    Express,
    /// The recipient collects the product.
    #[serde(rename = "SELF")]
    SelfPickup,
    /// Delivery by email.
    Email,
    /// Delivery by SMS.
    Sms,
}
