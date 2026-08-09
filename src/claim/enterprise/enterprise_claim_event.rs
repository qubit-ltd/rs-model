// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Immutable audit events for employer-sponsored claims.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::enterprise::EnterpriseClaimStatus;
use crate::claim::enterprise::EnterpriseClaimStatusGroup;

/// An auditable transition that records progress through an enterprise claim.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaimEvent {
    /// Typed identifier used when this enterprise claim event is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

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
