// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lightweight system-user information values.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::State;
use crate::contact::Phone;
use crate::person::Gender;

/// A compact user-information snapshot.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct UserInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Globally unique ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: String,

    /// Optional display name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub name: Option<String>,

    /// Optional nickname.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub nickname: Option<String>,

    /// Optional gender.
    pub gender: Option<Gender>,

    /// Optional ASCII avatar URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub avatar: Option<String>,

    /// Optional mobile number.
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Optional email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Lifecycle state.
    pub state: State,

    /// Whether this is a test user.
    pub test: bool,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            id: None,
            username: String::new(),
            name: None,
            nickname: None,
            gender: None,
            avatar: None,
            mobile: None,
            email: None,
            state: State::Normal,
            test: false,
            delete_time: None,
        }
    }
}
