// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted and thread-local application sessions.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use std::cell::RefCell;
use std::collections::HashSet;

use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::App;
use crate::commons::Token;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;
use crate::person::User;
use crate::person::UserInfo;
use crate::privilege::Role;
use super::Environment;
use super::Expired;

thread_local! {
    static CURRENT_SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };
    static SUPER_ADMIN_SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };
}

/// An application session with caller identity, permissions, and expiration.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact)]
#[serde(default)]
pub struct Session {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Optional application information.
    #[model(reference(target = App, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<StatefulInfo>,

    /// Optional user information.
    #[model(reference(target = User, target_field = info), opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,

    /// Optional organization information.
    #[model(reference(target = Organization, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<StatefulInfo>,

    /// Optional user access token.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Token>,

    /// Assigned role codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,

    /// Effective privilege names.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privileges: Vec<String>,

    /// Optional client environment.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,

    /// Optional UTC last-active timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_time: Option<DateTime<Utc>>,

    /// Optional expiration information.
    #[model(index)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<Expired>,

    /// Optional UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
}

impl Session {
    /// Clears current and super-administrator sessions for this thread.
    pub fn reset() {
        CURRENT_SESSION.with(|session| *session.borrow_mut() = None);
        SUPER_ADMIN_SESSION.with(|session| *session.borrow_mut() = None);
    }

    /// Returns a clone of this thread's current session, creating an empty one
    /// when necessary.
    #[must_use]
    pub fn current_session() -> Self {
        CURRENT_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.get_or_insert_with(Self::default).clone()
        })
    }

    /// Replaces this thread's current session.
    pub fn set_current_session(session: Self) {
        CURRENT_SESSION.with(|current| *current.borrow_mut() = Some(session));
    }

    /// Returns the current thread's application information.
    #[must_use]
    pub fn current_app() -> Option<StatefulInfo> {
        Self::current_session().app
    }

    /// Returns the current thread's user information.
    #[must_use]
    pub fn current_user() -> Option<UserInfo> {
        Self::current_session().user
    }

    /// Returns the current thread's user access token.
    #[must_use]
    pub fn current_user_token() -> Option<Token> {
        Self::current_session().token
    }

    /// Replaces the current thread's application information.
    pub fn set_current_app(app: Option<StatefulInfo>) {
        Self::with_current_session(|session| session.app = app);
    }

    /// Replaces the current thread's user information.
    pub fn set_current_user(user: Option<UserInfo>) {
        Self::with_current_session(|session| session.user = user);
    }

    /// Replaces the current thread's user access token.
    pub fn set_current_user_token(token: Option<Token>) {
        Self::with_current_session(|session| session.token = token);
    }

    /// Mutates this thread's current session and returns the closure result.
    pub fn with_current_session<R>(operation: impl FnOnce(&mut Self) -> R) -> R {
        CURRENT_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            operation(session.get_or_insert_with(Self::default))
        })
    }

    /// Returns a clone of this thread's super-administrator session.
    #[must_use]
    pub fn super_admin_session() -> Option<Self> {
        SUPER_ADMIN_SESSION.with(|session| session.borrow().clone())
    }

    /// Replaces this thread's optional super-administrator session.
    pub fn set_super_admin_session(session: Option<Self>) {
        SUPER_ADMIN_SESSION.with(|current| *current.borrow_mut() = session);
    }

    /// Clears this thread's super-administrator session.
    pub fn clear_super_admin_session() {
        SUPER_ADMIN_SESSION.with(|session| *session.borrow_mut() = None);
    }

    /// Reports whether this thread has a super-administrator session.
    #[must_use]
    pub fn is_super_admin_mode() -> bool {
        SUPER_ADMIN_SESSION.with(|session| session.borrow().is_some())
    }

    /// Reports whether this session contains `role`.
    #[must_use]
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|candidate| candidate == role)
    }

    /// Returns the current username when a user is attached.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.user.as_ref().map(|user| user.username.as_str())
    }

    /// Replaces role codes and effective privileges from role models.
    pub fn set_roles_and_privileges(&mut self, roles: &[Role]) {
        let role_codes: HashSet<_> = roles.iter().map(|role| role.code.clone()).collect();
        let privileges: HashSet<_> = roles
            .iter()
            .flat_map(|role| role.privileges.iter().cloned())
            .collect();
        self.roles = role_codes.into_iter().collect();
        self.privileges = privileges.into_iter().collect();
    }
}

impl Normalizable for Session {
    fn normalize(&mut self) {
        self.roles.normalize();
        self.privileges.normalize();
    }
}

/// Borrowed JSON-wire projection for a [`Session`].
#[derive(Serialize)]
struct SessionWire<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<&'a StatefulInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a UserInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<&'a StatefulInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a Token>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    roles: &'a Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    privileges: &'a Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<&'a Environment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_active_time: Option<&'a DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<&'a Expired>,

    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<&'a DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
}

impl Serialize for Session {
    /// Serializes all source-visible session fields and the computed username.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SessionWire {
            id: self.id,
            app: self.app.as_ref(),
            user: self.user.as_ref(),
            organization: self.organization.as_ref(),
            token: self.token.as_ref(),
            roles: &self.roles,
            privileges: &self.privileges,
            environment: self.environment.as_ref(),
            last_active_time: self.last_active_time.as_ref(),
            expired: self.expired.as_ref(),
            create_time: self.create_time.as_ref(),
            username: self.username(),
        }
        .serialize(serializer)
    }
}
