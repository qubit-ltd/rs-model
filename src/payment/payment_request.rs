// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Signed transaction submissions sent to payment gateways.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::payment::PaymentRequestTransformer;
use crate::settlement::Transaction;

/// A gateway-ready transaction plus the browser and server callback endpoints.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct PaymentRequest {
    /// Transaction data after provider-irrelevant internal fields have been removed.
    pub data: Transaction,

    /// Browser destination after the provider completes the customer flow.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub return_url: String,

    /// Server endpoint that receives the provider's asynchronous POST notification.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub notify_url: String,

    /// RSA signature over the message without this field; `None` means it has not been signed.
    #[model(text(min_chars = 1, max_chars = 2048, repertoire = ascii))]
    pub signature: Option<String>,
}

impl PaymentRequest {
    /// Removes internal-only transaction data before gateway submission.
    ///
    /// This mutates `data` in place and leaves only the fields required to identify and execute
    /// the payment.
    pub fn filter(&mut self) {
        PaymentRequestTransformer::transform(&mut self.data);
    }
}
