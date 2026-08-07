// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! WeChat JavaScript SDK configuration.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Parameters required to initialize the WeChat JavaScript SDK.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct WechatJsConfig {
    /// WeChat application identifier.
    pub app_id: String,

    /// WeChat timestamp text.
    pub timestamp: String,

    /// Per-request nonce.
    #[redact(level = "secret")]
    pub nonce_str: String,

    /// WeChat request signature.
    #[redact(level = "secret")]
    pub signature: String,
}
