// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Successful-login responses.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Token;
use crate::mixin::StatefulInfo;
use crate::person::UserInfo;
use crate::system::Session;

/// User, organization, token, privilege, and role data returned after login.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct LoginResponse {
    /// Logged-in user information.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,

    /// Optional organization information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<StatefulInfo>,

    /// User access token.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Token>,

    /// Effective privileges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privileges: Vec<String>,

    /// Assigned role codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}

impl LoginResponse {
    /// Creates a login response by cloning the corresponding session fields.
    #[must_use]
    pub fn from_session(session: &Session) -> Self {
        Self {
            user: session.user.clone(),
            organization: session.organization.clone(),
            token: session.token.clone(),
            privileges: session.privileges.clone(),
            roles: session.roles.clone(),
        }
    }
}
