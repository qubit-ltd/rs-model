// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim-item medical links.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Links an enterprise claim calculation item to a medical encounter.
#[derive(Clone, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
pub struct EnterpriseClaimItemMedical {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted claim-item identifier.
    pub claim_item_id: i64,
    /// Persisted claim-medical identifier.
    pub claim_medical_id: i64,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
