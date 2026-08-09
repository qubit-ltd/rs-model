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

use crate::mixin::StatefulInfo;
use crate::person::UserInfo;

/// Represents a user's role assignment in a specific application.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserRole {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Assigned user.
    pub user: UserInfo,

    /// Application that owns the role.
    pub app: StatefulInfo,

    /// Assigned role snapshot.
    pub role: StatefulInfo,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
