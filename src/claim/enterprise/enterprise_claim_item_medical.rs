// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Join records connecting enterprise-claim calculations to medical encounters.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Links one enterprise-claim allocation item to an encounter it accounts for.
#[derive(Model, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnterpriseClaimItemMedical {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim-item identifier.
    #[model(opaque)]
    pub claim_item_id: Id,

    /// Persisted claim-medical identifier.
    #[model(opaque)]
    pub claim_medical_id: Id,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
}
