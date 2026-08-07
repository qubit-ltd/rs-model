// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User medical-service package entitlements.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::service::UserMedicalItem;

/// A time-bounded medical package assigned to a user.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserMedicalPackage {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted user identifier.
    #[model(opaque)]
    pub user_id: Id,

    /// Persisted medical-package identifier.
    #[model(opaque)]
    pub medical_package_id: Id,

    /// Optional per-item entitlement state.
    pub user_medical_items: Option<Vec<UserMedicalItem>>,

    /// UTC validity start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub valid_from: DateTime<Utc>,

    /// UTC validity end timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub valid_until: DateTime<Utc>,
}
