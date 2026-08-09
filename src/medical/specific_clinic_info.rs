// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! HIS data recorded for special-disease outpatient visits.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// Source-system details for an outpatient visit handled under a special-disease
/// program.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpecificClinicInfo {
    /// Source-system business sequence number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Optional remark.
    pub remark: Option<String>,

    /// Department visited by the patient.
    #[model(opaque)]
    pub department: Info,

    /// Optional outpatient record number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub record_number: Option<String>,

    /// UTC visit timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub visit_time: DateTime<Utc>,

    /// Optional special-disease information.
    #[model(opaque)]
    pub special_disease: Option<Info>,
}
