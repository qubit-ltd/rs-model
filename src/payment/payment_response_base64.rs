// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Base64-encoded payment responses.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// A payment gateway response serialized as a Base64 string.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PaymentResponseBase64 {
    /// Base64-encoded response payload.
    #[model(sensitive(token), text(min_chars = 1, repertoire = ascii))]
    #[redact(level = "secret")]
    pub data: String,
}
