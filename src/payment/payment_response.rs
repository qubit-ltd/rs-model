// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Signed payment gateway responses.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::payment::Payment;

/// A payment provider response together with callback URLs and its signature.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PaymentResponse {
    /// Provider-side payment record.
    pub data: Payment,

    /// Optional callback URL to which a user returns.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub return_url: Option<String>,

    /// Optional endpoint that receives provider notifications.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub notify_url: Option<String>,

    /// RSA signature of the JSON message excluding this field.
    #[model(text(min_chars = 1, max_chars = 2048, repertoire = ascii))]
    pub signature: String,
}
