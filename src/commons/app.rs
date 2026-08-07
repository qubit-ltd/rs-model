// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::mixin::StatefulInfo;
use crate::person::UserInfo;
use super::AuthorizeRecord;
use super::State;
use super::Token;

/// Represents the App domain type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Redact, Serialize)]
pub struct App {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The code value associated with this model.
    pub code: String,

    /// The name value associated with this model.
    pub name: String,

    /// The organization value associated with this model.
    pub organization: StatefulInfo,

    /// The category value associated with this model.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// The state value associated with this model.
    pub state: State,

    /// The icon value associated with this model.
    pub icon: Option<String>,

    /// The url value associated with this model.
    pub url: Option<String>,

    /// The description value associated with this model.
    pub description: Option<String>,

    /// The comment value associated with this model.
    pub comment: Option<String>,

    /// The security_key value associated with this model.
    #[redact(level = "secret")]
    pub security_key: Option<String>,

    /// The token value associated with this model.
    pub token: Option<Token>,

    /// The last_authorize value associated with this model.
    pub last_authorize: AuthorizeRecord,

    /// The default_user value associated with this model.
    pub default_user: Option<UserInfo>,

    /// The predefined value associated with this model.
    pub predefined: bool,

    /// The create_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// The modify_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// The delete_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
