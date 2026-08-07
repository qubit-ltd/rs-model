// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Application-authentication parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Location;
use crate::system::Platform;

/// Credentials and client environment used to authenticate an application.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct AppAuthenticateParams {
    /// Application code.
    pub code: String,

    /// Application security key.
    #[redact(level = "secret")]
    pub security_key: String,

    /// Optional client platform.
    pub platform: Option<Platform>,

    /// Optional device identifier.
    #[redact(level = "secret")]
    pub udid: Option<String>,

    /// Optional push-notification token.
    #[redact(level = "secret")]
    pub push_token: Option<String>,

    /// Optional client location.
    #[redact(nested)]
    pub location: Option<Location>,
}
