// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Application-authentication parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{contact::Location, system::Platform};

/// Credentials and client environment used to authenticate an application.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct AppAuthenticateParams {
    /// Application code.
    pub code: String,
    /// Application security key.
    #[redact(level = "secret")]
    pub security_key: String,
    /// Optional client platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    /// Optional device identifier.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    /// Optional push-notification token.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_token: Option<String>,
    /// Optional client location.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
