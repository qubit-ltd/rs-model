// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order submission payloads and their callback endpoints.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::order::Order;

/// An order submitted to a checkout flow with browser and server callback URLs.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderSubmitRequest {
    /// The complete order aggregate submitted for checkout.
    pub order: Order,

    /// Browser destination after checkout returns control to the client.
    pub return_url: String,

    /// Server endpoint for asynchronous checkout notifications.
    pub notify_url: String,
}
