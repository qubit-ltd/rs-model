// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Client-authenticated requests that submit a refund to a payment flow.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::Client;

/// Refund authorization data together with the browser and server callback endpoints.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ClientRefundSubmitRequest {
    /// Sensitive one-time code authorizing this client refund.
    #[model(sensitive(token))]
    #[redact(level = "secret")]
    pub verify_code: String,

    /// Client identity and payment details that receive the refunded amount.
    pub client: Client,

    /// Browser destination after the refund flow returns control to the client.
    pub return_url: String,

    /// Server endpoint for asynchronous refund notifications.
    pub notify_url: String,
}
