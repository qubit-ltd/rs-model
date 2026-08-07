// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User medical-service item entitlements.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::service::UserServiceState;

/// A user's remaining quantity and state for one medical service item.
#[derive(Clone, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
pub struct UserMedicalItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted medical-item identifier.
    pub medical_item_id: i64,

    /// Remaining number of uses.
    pub count: i32,

    /// Current entitlement or appointment state.
    pub state: UserServiceState,
}
