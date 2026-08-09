// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Immutable audit events for individual-claim processing.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::claim::InsuranceClaimStatus;
use crate::claim::InsuranceClaimStatusGroup;

/// An auditable transition that records who moved an individual claim to a
/// particular workflow state.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct InsuranceClaimEvent {
    /// Typed identifier used when this claim event is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Detailed claim state recorded by this event.
    pub status: InsuranceClaimStatus,

    /// High-level state group recorded by this event.
    pub status_group: InsuranceClaimStatusGroup,

    /// Operator's name.
    pub operator_name: String,

    /// Operator's mobile number.
    #[redact(level = "secret")]
    pub operator_mobile: String,

    /// Human-readable transition detail retained in the claim audit trail.
    pub detail: String,

    /// UTC time at which the workflow transition was recorded.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
