// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Client refund submission messages.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::Client;

/// Verification and callback data submitted for a client refund.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ClientRefundSubmitRequest {
    /// One-time refund verification code.
    #[model(sensitive(token))]
    #[redact(level = "secret")]
    pub verify_code: String,

    /// Client receiving the refund.
    pub client: Client,

    /// Browser return URL.
    pub return_url: String,

    /// Server notification URL.
    pub notify_url: String,
}
