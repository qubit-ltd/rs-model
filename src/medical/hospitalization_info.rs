// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! HIS data for an inpatient admission and discharge episode.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// Source-system record of an inpatient stay, from admission through discharge.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HospitalizationInfo {
    /// Source-system business sequence number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Source-system note, absent when the inpatient episode has no remark.
    pub remark: Option<String>,

    /// Admission department.
    #[model(opaque)]
    pub admission_department: Info,

    /// Discharge department.
    #[model(opaque)]
    pub discharge_department: Info,

    /// Inpatient patient number, absent when not issued by the HIS source.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub patient_number: Option<String>,

    /// Inpatient medical-record number, absent when the HIS source omitted it.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub record_number: Option<String>,

    /// Inpatient ward, absent when ward placement is not recorded.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub ward: Option<String>,

    /// Inpatient bed number, absent when bed placement is not recorded.
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
