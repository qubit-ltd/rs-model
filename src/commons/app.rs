// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use super::{
    AuthorizeRecord,
    State,
    Token,
};
use crate::{
    mixin::StatefulInfo,
    person::UserInfo,
};
use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// Represents the App domain type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Redact, Serialize)]
pub struct App {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    /// The code value associated with this model.
    pub code: String,
    /// The name value associated with this model.
    pub name: String,
    /// The organization value associated with this model.
    pub organization: StatefulInfo,
    #[model(opaque)]
    /// The category value associated with this model.
    pub category: Option<qubit_mixin::InfoWithEntity>,
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
    #[redact(level = "secret")]
    /// The security_key value associated with this model.
    pub security_key: Option<String>,
    /// The token value associated with this model.
    pub token: Option<Token>,
    /// The last_authorize value associated with this model.
    pub last_authorize: AuthorizeRecord,
    /// The default_user value associated with this model.
    pub default_user: Option<UserInfo>,
    /// The predefined value associated with this model.
    pub predefined: bool,
    #[model(time(precision=second,normalization=utc))]
    /// The create_time value associated with this model.
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    /// The modify_time value associated with this model.
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The delete_time value associated with this model.
    pub delete_time: Option<DateTime<Utc>>,
}
