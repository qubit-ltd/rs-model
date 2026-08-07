// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Special outpatient clinic visit information.

use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Hospital-system information for a special outpatient visit.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
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
