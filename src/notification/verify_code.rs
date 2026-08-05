// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted mobile and email verification codes.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::notification::VerifyScene;

/// A verification code delivered to a mobile number or email address.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct VerifyCode {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Tenant that owns the issuing application.
    pub tenant: StatefulInfo,
    /// Application that issued the code.
    pub app: StatefulInfo,
    /// Optional destination mobile number.
    pub mobile: Option<Phone>,
    /// Optional destination email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,
    /// Verification scenario.
    pub scene: VerifyScene,
    /// Secret verification token.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub code: String,
    /// Message delivered with the code.
    pub message: String,
    /// Whether this code has already been verified.
    pub verified: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}

impl VerifyCode {
    /// Replaces the verification token with the Java TOKEN-level masked form.
    ///
    /// Tokens longer than eight characters retain four characters at each end
    /// with `...` in the middle. Shorter tokens are replaced entirely by `*`.
    pub fn desensitize(&mut self) {
        let character_count = self.code.chars().count();
        if character_count > 8 {
            let prefix: String = self.code.chars().take(4).collect();
            let suffix: String = self
                .code
                .chars()
                .skip(character_count.saturating_sub(4))
                .collect();
            self.code = format!("{prefix}...{suffix}");
        } else {
            self.code = "*".repeat(character_count);
        }
    }
}
