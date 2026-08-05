// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim workflow events.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::claim::enterprise::{
    EnterpriseClaimStatus,
    EnterpriseClaimStatusGroup,
};

/// A state transition recorded for an enterprise insurance claim.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct EnterpriseClaimEvent {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted claim identifier.
    pub claim_id: i64,
    /// Detailed enterprise claim state.
    pub status: EnterpriseClaimStatus,
    /// High-level enterprise claim state group.
    pub status_group: EnterpriseClaimStatusGroup,
    /// Operator's name.
    pub operator_name: String,
    /// Event details.
    pub detail: String,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
