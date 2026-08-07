// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual claim workflow events.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::claim::InsuranceClaimStatus;
use crate::claim::InsuranceClaimStatusGroup;

/// A state transition recorded for an individual insurance claim.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct InsuranceClaimEvent {
    /// Optional persisted identifier.
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

    /// Event details.
    pub detail: String,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
