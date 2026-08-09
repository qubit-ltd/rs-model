// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Registered third-party applications and their authorization context.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AuthorizeRecord;
use super::State;
use super::Token;
use crate::mixin::StatefulInfo;
use crate::person::UserInfo;

/// A third-party application registered on the platform.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct App {
    /// Platform-assigned identifier for this application.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Immutable, globally unique application code.
    pub code: String,

    /// Application name, unique within its owning organization.
    pub name: String,

    /// Required stateful reference to the organization that owns the application.
    pub organization: StatefulInfo,

    /// Optional category used to classify the application; `None` means uncategorized.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Lifecycle state that determines whether the application is available.
    pub state: State,

    /// Optional ASCII icon URI; `None` means no icon has been supplied.
    pub icon: Option<String>,

    /// Optional ASCII homepage URL; `None` means no homepage is recorded.
    pub url: Option<String>,

    /// Optional user-facing description maintained by the application owner.
    pub description: Option<String>,

    /// Optional administrator-only note, distinct from the user-facing description.
    pub comment: Option<String>,

    /// Optional secret used to authenticate the application; persisted values are salted hashes.
    #[redact(level = "secret")]
    pub security_key: Option<String>,

    /// Optional current access token issued to this application.
    pub token: Option<Token>,

    /// Last authorization time and consecutive authorization-failure count.
    pub last_authorize: AuthorizeRecord,

    /// User assumed for calls without an authenticated user; `None` disables this fallback.
    pub default_user: Option<UserInfo>,

    /// Whether this application is platform-provided reference data.
    pub predefined: bool,

    /// Creation time in UTC, stored with second precision.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// Most recent modification time in UTC, or `None` if never modified.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, or `None` while the application remains active.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
