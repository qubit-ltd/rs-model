// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verification tokens issued to mobile numbers or email addresses.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::notification::VerifyScene;

/// A one-time verification token issued for a specific application scenario.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct VerifyCode {
    /// Database identifier; the default value denotes an unpersisted token record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Tenant reference for the application that issued the token.
    pub tenant: StatefulInfo,

    /// Application reference responsible for issuing and validating the token.
    pub app: StatefulInfo,

    /// Mobile destination, or `None` when no mobile destination was recorded.
    pub mobile: Option<Phone>,

    /// ASCII email destination, or `None` when no email destination was recorded.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,

    /// Operation for which the recipient must present this token.
    pub scene: VerifyScene,

    /// Secret token compared during verification; never expose it to untrusted callers.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub code: String,

    /// Notification message delivered with the token.
    pub message: String,

    /// Whether successful verification has already consumed this token.
    pub verified: bool,

    /// UTC instant, rounded to seconds, when the token was issued.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}

impl VerifyCode {
    /// Masks this token in place before it is exposed.
    ///
    /// Tokens longer than eight characters retain four characters at each end;
    /// shorter tokens are replaced entirely with asterisks.
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
