// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Assignment of a user to an application role.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use super::Role;
use crate::commons::App;
use crate::mixin::StatefulInfo;
use crate::person::User;
use crate::person::UserInfo;

/// Represents a user's role assignment in a specific application.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(name = "user_role_assignment", fields(user, app, role)))]
pub struct UserRole {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Assigned user.
    #[model(reference(target = User, target_field = info), index)]
    pub user: UserInfo,

    /// Application that owns the role.
    #[model(reference(target = App, target_field = info), index)]
    pub app: StatefulInfo,

    /// Assigned role snapshot.
    #[model(reference(target = Role, target_field = info), index)]
    pub role: StatefulInfo,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
