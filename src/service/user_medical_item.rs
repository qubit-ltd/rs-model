// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User medical-service item entitlements.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::service::UserServiceState;

/// A user's remaining quantity and state for one medical service item.
#[derive(Model, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserMedicalItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted medical-item identifier.
    #[model(opaque)]
    pub medical_item_id: Id,

    /// Remaining number of uses.
    pub count: i32,

    /// Current entitlement or appointment state.
    pub state: UserServiceState,
}
