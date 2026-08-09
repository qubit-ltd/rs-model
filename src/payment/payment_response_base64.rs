// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Opaque Base64 envelopes used for payment-provider responses.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A sensitive payment-gateway response represented as Base64 text.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct PaymentResponseBase64 {
    /// The Base64-encoded provider payload; callers must decode it before interpreting it.
    #[model(sensitive(token), text(min_chars = 1, repertoire = ascii))]
    #[redact(level = "secret")]
    pub data: String,
}
