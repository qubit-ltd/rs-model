// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! HIS data for emergency outpatient encounters.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// Source-system record for a patient's emergency outpatient visit.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EmergentClinicInfo {
    /// Source-system business sequence number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Source-system note, absent when the emergency visit has no remark.
    pub remark: Option<String>,

    /// Department visited by the patient.
    #[model(opaque)]
    pub department: Info,

    /// Emergency outpatient record number, absent when the HIS source omitted it.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub record_number: Option<String>,

    /// UTC visit timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub visit_time: DateTime<Utc>,
}
