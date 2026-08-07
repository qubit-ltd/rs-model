// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! System-user domain models.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::AuthorizeRecord;
use crate::commons::State;
use crate::commons::VerifyState;
use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::person::Gender;

/// A system user with authentication, contact, and lifecycle data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct User {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Globally unique ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: String,

    /// Stored password hash, which must never appear in diagnostic output.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub password: String,

    /// Optional real name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: Option<String>,

    /// Optional nickname.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub nickname: Option<String>,

    /// Optional gender.
    pub gender: Option<Gender>,

    /// Optional mobile number.
    pub mobile: Option<Phone>,

    /// Optional mobile verification state.
    pub mobile_verified: Option<VerifyState>,

    /// Optional ASCII email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,

    /// Optional email verification state.
    pub email_verified: Option<VerifyState>,

    /// Optional ASCII avatar URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub avatar: Option<String>,

    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional self-description.
    pub description: Option<String>,

    /// Optional organization information.
    pub organization: Option<StatefulInfo>,

    /// Current lifecycle state.
    pub state: State,

    /// Most recent authorization record.
    pub last_login: AuthorizeRecord,

    /// Whether the user must change the password.
    pub change_password: bool,

    /// Optional UTC validity start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub valid_time: Option<DateTime<Utc>>,

    /// Optional UTC expiration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub expired_time: Option<DateTime<Utc>>,

    /// Optional comment.
    pub comment: Option<String>,

    /// Whether the user is predefined reference data.
    pub predefined: bool,

    /// Whether this is a test user.
    pub test: bool,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
