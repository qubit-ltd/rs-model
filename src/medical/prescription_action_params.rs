// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Input captured when a participant advances a prescription workflow.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::Entity;
use crate::medical::PrescriptionAction;

/// Audit context for a prescription transition, including its actor and source
/// payload.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrescriptionActionParams {
    /// Action applied to the prescription.
    pub action: PrescriptionAction,

    /// Entity classification of the actor.
    pub actor_type: Entity,

    /// Actor's source-domain code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub actor_code: String,

    /// Actor's display name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub actor_name: String,

    /// Action details.
    pub description: String,

    /// UTC action timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub timestamp: DateTime<Utc>,

    /// Optional source-order key-value payload.
    #[model(sequence(max_items = 16), opaque)]
    pub payload: Option<Vec<(String, String)>>,
}
