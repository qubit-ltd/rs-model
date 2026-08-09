// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Signed configuration used to initialize the WeChat JavaScript SDK.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Per-page signature parameters accepted by the WeChat JavaScript SDK.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct WechatJsConfig {
    /// WeChat application identifier for the signing context.
    pub app_id: String,

    /// Timestamp supplied to WeChat as text, using its expected wire format.
    pub timestamp: String,

    /// One-time nonce paired with the signature.
    #[redact(level = "secret")]
    pub nonce_str: String,

    /// Signature that authorizes SDK initialization for the current request.
    #[redact(level = "secret")]
    pub signature: String,
}
