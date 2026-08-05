// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Messages submitted to payment gateways.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{payment::PaymentRequestTransformer, settlement::Transaction};

/// A signed transaction request submitted to a payment gateway.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PaymentRequest {
    /// Transaction data to submit after filtering.
    pub data: Transaction,
    /// URL to which a user returns after payment.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub return_url: String,
    /// URL that receives the payment provider's POST notification.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub notify_url: String,
    /// Optional RSA signature of the JSON message excluding this field.
    #[model(text(min_chars = 1, max_chars = 2048, repertoire = ascii))]
    pub signature: Option<String>,
}

impl PaymentRequest {
    /// Removes internal fields from the transaction before submission.
    pub fn filter(&mut self) {
        PaymentRequestTransformer::transform(&mut self.data);
    }
}
