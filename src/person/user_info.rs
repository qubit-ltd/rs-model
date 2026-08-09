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
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::State;
use crate::contact::Phone;
use crate::person::Gender;

/// A compact user-information snapshot.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct UserInfo {
    /// Database identifier of the referenced user; default denotes an unsaved user.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII user name.
    #[model(index, unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: String,

    /// Real name suitable for compact profile displays.
    #[model(index, text(min_chars = 1, max_chars = 64))]
    pub name: Option<String>,

    /// Informal display name suitable for compact profile displays.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub nickname: Option<String>,

    /// Gender copied for identity displays where the source provided it.
    #[model(index)]
    pub gender: Option<Gender>,

    /// Avatar URI used by clients rendering the compact user identity.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub avatar: Option<String>,

    /// Mobile contact channel included when the projection must contact the user.
    #[model(index, unique)]
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Email contact channel included when the projection must contact the user.
    #[model(index, unique(ignore_case), sensitive(redact), text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Lifecycle state.
    #[model(index)]
    pub state: State,

    /// Marks a synthetic user so consumers can exclude it from live interactions.
    #[model(index)]
    pub test: bool,

    /// Soft-deletion time copied from the user record; absence means the user is active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            id: Id::default(),
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
