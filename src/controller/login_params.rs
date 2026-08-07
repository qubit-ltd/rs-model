// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Login-operation parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::person::SocialNetwork;
use crate::system::Environment;

/// Credentials and client environment accepted by the login operation.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct LoginParams {
    /// Optional username identity.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub username: Option<String>,

    /// Optional email identity.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Optional mobile identity.
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Optional password credential.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub password: Option<String>,

    /// Optional one-time verification code.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: Option<String>,

    /// Optional social-network provider.
    pub social_network: Option<SocialNetwork>,

    /// Optional social-network application identifier.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub app_id: Option<String>,

    /// Optional social-network open identifier.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub open_id: Option<String>,

    /// Optional login client environment.
    #[redact(nested)]
    pub environment: Option<Environment>,
}

impl LoginParams {
    /// Replaces plaintext credentials with the source desensitized marker.
    pub fn desensitize(&mut self) {
        self.password = Some("--------".into());
        self.open_id = Some("--------".into());
    }
}
