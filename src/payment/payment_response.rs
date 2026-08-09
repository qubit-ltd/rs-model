// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Signed provider responses returned from payment execution.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::payment::Payment;

/// Provider execution data together with the callback endpoints and response signature.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct PaymentResponse {
    /// Provider-side payment record containing the outcome and provider identifiers.
    pub data: Payment,

    /// Browser return endpoint, or `None` when the flow has no browser callback.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub return_url: Option<String>,

    /// Server notification endpoint, or `None` when no asynchronous callback is used.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub notify_url: Option<String>,

    /// RSA signature over the response JSON excluding this field.
    #[model(text(min_chars = 1, max_chars = 2048, repertoire = ascii))]
    pub signature: String,
}
