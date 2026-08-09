// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Successful-login responses.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Token;
use crate::mixin::StatefulInfo;
use crate::person::UserInfo;
use crate::system::Session;

/// User, organization, token, privilege, and role data returned after login.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct LoginResponse {
    /// Logged-in user information.
    #[redact(nested)]
    pub user: Option<UserInfo>,

    /// Organization context selected or resolved for the authenticated session.
    pub organization: Option<StatefulInfo>,

    /// User access token.
    #[redact(nested)]
    pub token: Option<Token>,

    /// Deduplicated privilege names granted to the authenticated session.
    pub privileges: Vec<String>,

    /// Role codes from which the effective privileges were derived.
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
