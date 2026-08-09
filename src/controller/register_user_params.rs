// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! User-registration parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;
use crate::person::Gender;
use crate::person::SocialNetwork;
use crate::person::User;
use crate::person::UserInfo;
use crate::system::Environment;

/// Account, profile, social identity, and client data used for registration.
#[allow(clippy::duplicated_attributes)]
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(
    name = "register_social_identity",
    fields(social_network, app_id, open_id),
    ignore_case(open_id)
))]
pub struct RegisterUserParams {
    /// Globally unique username.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64))]
    pub username: String,

    /// Plaintext password supplied during registration.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub password: String,

    /// One-time code required when registration verifies an email or mobile channel.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: Option<String>,

    /// Real name to display on the new account profile.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub name: Option<String>,

    /// Informal display name distinct from the account username.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub nickname: Option<String>,

    /// Self-reported gender to seed the new user's profile.
    pub gender: Option<Gender>,

    /// Avatar location shown for the user in account-facing experiences.
    #[model(text(min_chars = 1, max_chars = 512))]
    pub avatar: Option<String>,

    /// Mobile contact number to attach to the account and optionally verify.
    #[model(unique)]
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Email contact address to attach to the account and optionally verify.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Provider for a social identity linked during registration.
    pub social_network: Option<SocialNetwork>,

    /// Provider application identifier that scopes the linked social account.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub app_id: Option<String>,

    /// Provider-issued subject identifier for the linked social account.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub open_id: Option<String>,

    /// Organization context under which the account is initially registered.
    #[model(reference(target = Organization, target_field = info), opaque)]
    pub organization: Option<StatefulInfo>,

    /// Device and network context captured for registration risk assessment.
    #[redact(nested)]
    pub environment: Option<Environment>,
}

impl RegisterUserParams {
    /// Creates registration parameters from a complete user record.
    #[must_use]
    pub fn from_user(user: &User) -> Self {
        Self {
            username: user.username.clone(),
            password: user.password.clone(),
            name: user.name.clone(),
            nickname: user.nickname.clone(),
            gender: user.gender,
            avatar: user.avatar.clone(),
            mobile: user.mobile.clone(),
            email: user.email.clone(),
            organization: user.organization.clone(),
            ..Self::default()
        }
    }

    /// Creates registration parameters from a lightweight user view.
    #[must_use]
    pub fn from_user_info(user: &UserInfo) -> Self {
        Self {
            username: user.username.clone(),
            name: user.name.clone(),
            nickname: user.nickname.clone(),
            gender: user.gender,
            avatar: user.avatar.clone(),
            mobile: user.mobile.clone(),
            email: user.email.clone(),
            ..Self::default()
        }
    }

    /// Replaces plaintext credentials with the source desensitized marker.
    pub fn desensitize(&mut self) {
        self.password = "--------".into();
        self.open_id = Some("--------".into());
    }
}
