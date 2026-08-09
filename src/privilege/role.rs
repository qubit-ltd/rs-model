// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Application role model.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::App;
use crate::commons::State;
use crate::mixin::StatefulInfo;

/// Represents a named role and its privileges within one application.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(
    unique(name = "role_app_code", fields(app, code), ignore_case(code)),
    unique(name = "role_app_name", fields(app, name), ignore_case(name))
)]
pub struct Role {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application to which the role belongs.
    #[model(reference(target = App, target_field = info), index)]
    pub app: StatefulInfo,

    /// ASCII code unique within `app`.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Name unique within `app`.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional role description.
    pub description: Option<String>,

    /// Whether this is the guest role.
    #[model(index)]
    pub guest: Option<bool>,

    /// Whether this is the application's basic role.
    #[model(index)]
    pub basic: Option<bool>,

    /// ASCII permission names.
    #[model(sequence(min_items = 1, max_items = 256), element(text(repertoire = ascii)))]
    pub privileges: Vec<String>,

    /// Lifecycle state.
    #[model(index)]
    pub state: State,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Role {
    /// Collects privileges from `roles` while preserving first-seen order.
    #[must_use]
    pub fn collect_privileges(roles: &[Self]) -> Vec<String> {
        let mut result = Vec::new();
        for role in roles {
            for privilege in &role.privileges {
                if !result.contains(privilege) {
                    result.push(privilege.clone());
                }
            }
        }
        result
    }
}
