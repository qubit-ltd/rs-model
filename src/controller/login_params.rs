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
    /// Account username supplied as one possible login identifier.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub username: Option<String>,

    /// Verified account email supplied as an alternative login identifier.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Verified mobile number supplied as an alternative login identifier.
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Password proof for password-based authentication; absent for other login flows.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub password: Option<String>,

    /// One-time code that proves control of the selected email or mobile channel.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: Option<String>,

    /// Social provider used when the caller authenticates through a linked account.
    pub social_network: Option<SocialNetwork>,

    /// Provider application identifier that scopes the supplied social `open_id`.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub app_id: Option<String>,

    /// Provider-issued subject identifier for the authenticating social account.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub open_id: Option<String>,

    /// Client device and network context recorded with the authentication attempt.
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
