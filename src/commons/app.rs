// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use super::{AuthorizeRecord, State, Token};
use crate::{mixin::StatefulInfo, person::UserInfo};
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct App {
    #[model(identifier)]
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub organization: StatefulInfo,
    #[model(opaque)]
    pub category: Option<qubit_mixin::InfoWithEntity>,
    pub state: State,
    pub icon: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub comment: Option<String>,
    #[redact(level = "secret")]
    pub security_key: Option<String>,
    pub token: Option<Token>,
    pub last_authorize: AuthorizeRecord,
    pub default_user: Option<UserInfo>,
    pub predefined: bool,
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
