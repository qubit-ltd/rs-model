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
use super::Category;
use super::State;
use super::Token;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;
use crate::person::User;
use crate::person::UserInfo;

/// A third-party application registered on the platform.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(unique(
    name = "app_organization_name",
    fields(organization, name),
    ignore_case(name)
))]
pub struct App {
    /// Platform-assigned identifier for this application.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Immutable, globally unique application code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Application name, unique within its owning organization.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Required stateful reference to the organization that owns the application.
    #[model(reference(target = Organization, target_field = info), index)]
    pub organization: StatefulInfo,

    /// Optional category used to classify the application; `None` means uncategorized.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub category: Option<InfoWithEntity>,

    /// Lifecycle state that determines whether the application is available.
    #[model(index)]
    pub state: State,

    /// Optional ASCII icon URI; `None` means no icon has been supplied.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional ASCII homepage URL; `None` means no homepage is recorded.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional user-facing description maintained by the application owner.
    pub description: Option<String>,

    /// Optional administrator-only note, distinct from the user-facing description.
    pub comment: Option<String>,

    /// Optional secret used to authenticate the application; persisted values are salted hashes.
    #[model(text(min_chars = 1, max_chars = 4096, repertoire = ascii))]
    #[redact(level = "secret")]
    pub security_key: Option<String>,

    /// Optional current access token issued to this application.
    #[model(opaque)]
    pub token: Option<Token>,

    /// Last authorization time and consecutive authorization-failure count.
    #[model(index)]
    pub last_authorize: AuthorizeRecord,

    /// User assumed for calls without an authenticated user; `None` disables this fallback.
    #[model(reference(target = User, target_field = info))]
    pub default_user: Option<UserInfo>,

    /// Whether this application is platform-provided reference data.
    #[model(index)]
    pub predefined: bool,

    /// Creation time in UTC, stored with second precision.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Most recent modification time in UTC, or `None` if never modified.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, or `None` while the application remains active.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
