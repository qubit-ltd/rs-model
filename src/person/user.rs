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
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::AuthorizeRecord;
use crate::commons::State;
use crate::commons::VerifyState;
use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::person::Gender;

/// A system user with authentication, contact, and lifecycle data.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct User {
    /// Database identifier for this account; default denotes an account not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: String,

    /// Stored password hash, which must never appear in diagnostic output.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub password: String,

    /// Real name associated with the account for profile and administrative displays.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: Option<String>,

    /// Informal display name selected by the account holder.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub nickname: Option<String>,

    /// Gender supplied for the account profile.
    pub gender: Option<Gender>,

    /// Mobile contact channel that can be used for login or verification.
    pub mobile: Option<Phone>,

    /// Verification outcome for the mobile channel before it can be trusted for access flows.
    pub mobile_verified: Option<VerifyState>,

    /// Email contact channel that can be used for login or verification.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,

    /// Verification outcome for the email channel before it can be trusted for access flows.
    pub email_verified: Option<VerifyState>,

    /// Avatar URI shown with the account in user-facing interfaces.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub avatar: Option<String>,

    /// Personal or organization web address published on the account profile.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Account holder's profile biography or self-description.
    pub description: Option<String>,

    /// Organization context to which this account is currently associated.
    pub organization: Option<StatefulInfo>,

    /// Account lifecycle state controlling whether authentication is allowed.
    pub state: State,

    /// Most recent authorization record.
    pub last_login: AuthorizeRecord,

    /// Requires the account holder to replace the current password at the next login.
    pub change_password: bool,

    /// UTC instant before which the account must not authenticate.
    #[model(time(precision = second, normalization = utc))]
    pub valid_time: Option<DateTime<Utc>>,

    /// UTC instant after which the account must no longer authenticate.
    #[model(time(precision = second, normalization = utc))]
    pub expired_time: Option<DateTime<Utc>>,

    /// Internal administrator note about account handling or provenance.
    pub comment: Option<String>,

    /// Marks a platform-provided account that has special administrative semantics.
    pub predefined: bool,

    /// Marks synthetic account data excluded from production operations.
    pub test: bool,

    /// UTC instant when the account was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent account update.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC instant of account soft deletion; absence means the account remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
