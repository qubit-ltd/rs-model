// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Hospitalization information.

use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Hospital-system information for an inpatient stay.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct HospitalizationInfo {
    /// Source-system business sequence number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Optional remark.
    pub remark: Option<String>,

    /// Admission department.
    #[model(opaque)]
    pub admission_department: Info,

    /// Discharge department.
    #[model(opaque)]
    pub discharge_department: Info,

    /// Optional inpatient patient number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub patient_number: Option<String>,

    /// Optional inpatient record number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub record_number: Option<String>,

    /// Optional ward.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub ward: Option<String>,

    /// Optional bed number.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub bed: Option<String>,

    /// UTC admission timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub admission_time: DateTime<Utc>,

    /// UTC discharge timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub discharge_time: DateTime<Utc>,

    /// Number of inpatient days.
    pub days: i32,
}
