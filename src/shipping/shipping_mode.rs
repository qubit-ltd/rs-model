// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Ways an order can be delivered or collected.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The fulfilment channel used to make a product available to its recipient.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingMode {
    /// No separate delivery step is required.
    None,
    /// Delivery through an express carrier.
    Express,
    /// The recipient collects the product in person.
    #[serde(rename = "SELF")]
    SelfPickup,
    /// Delivery through email.
    Email,
    /// Delivery through SMS.
    Sms,
}
