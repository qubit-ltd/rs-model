// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User medical-service package entitlements.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::service::UserMedicalItem;

/// A time-bounded medical package assigned to a user.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct UserMedicalPackage {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted user identifier.
    pub user_id: i64,
    /// Persisted medical-package identifier.
    pub medical_package_id: i64,
    /// Optional per-item entitlement state.
    pub user_medical_items: Option<Vec<UserMedicalItem>>,
    /// UTC validity start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub valid_from: DateTime<Utc>,
    /// UTC validity end timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub valid_until: DateTime<Utc>,
}
