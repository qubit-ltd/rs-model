// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Clinical prescription content that participants review and sign.

use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::commons::DictEntryInfo;
use crate::medical::Diagnosis;
use crate::medical::Dosage;
use crate::medical::MedicalType;
use crate::medical::Patient;
use crate::medical::PrescriptionItem;
use crate::organization::EmployeeInfo;

/// The clinical facts of a prescription, separated from its mutable workflow
/// record so they can be digitally signed and verified as one stable payload.
///
/// The fields model electronic outpatient and traditional-medicine
/// prescriptions; workflow participants are recorded by [`Prescription`].
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrescriptionContent {
    /// Prescription sequence number within the issuing organization.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,

    /// Dictionary category such as Western medicine, Chinese medicine, or
    /// health products.
    pub category: DictEntryInfo,

    /// Dictionary subtype; Chinese and Western prescriptions use different
    /// subtype vocabularies.
    pub r#type: DictEntryInfo,

    /// Dictionary flow direction, for example in-hospital or external filling.
    pub direction: DictEntryInfo,

    /// Payer-source dictionary entry, such as employee insurance, resident
    /// insurance, public expense, or self-pay.
    pub cost_source: DictEntryInfo,

    /// Medical encounter classification.
    pub medical_type: MedicalType,

    /// Hospital visit or inpatient sequence number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub medical_number: String,

    /// Electronic medical-record sequence number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub record_number: String,

    /// Issuing hospital.
    #[model(opaque)]
    pub hospital: Info,

    /// Issuing department.
    #[model(opaque)]
    pub department: Info,

    /// Clinical-subject dictionary entry.
    pub subject: DictEntryInfo,

    /// Inpatient ward; omitted for outpatient prescriptions.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub ward: Option<String>,

    /// Inpatient bed number; omitted for outpatient prescriptions.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub bed: Option<String>,

    /// Patient receiving the prescription.
    pub patient: Patient,

    /// Patient's chief complaint.
    pub complaint: String,

    /// Ranked diagnoses.
    #[model(sequence(min_items = 1, max_items = 8))]
    pub diagnoses: Vec<Diagnosis>,

    /// Patient weight in kilograms, recorded when clinically required (for
    /// example, for newborn dosing).
    pub weight: Option<i32>,

    /// Patient allergy history.
    pub allergic_history: String,

    /// Local prescription issue timestamp.
    #[model(time(precision = second))]
    pub issue_time: NaiveDateTime,

    /// Number of days for which the prescription is valid.
    pub validity_days: i32,

    /// Prescribed line items.
    #[model(sequence(min_items = 1, max_items = 10))]
    pub items: Vec<PrescriptionItem>,

    /// Course-level directions for a traditional-medicine prescription whose
    /// items are individual ingredients rather than separately administered
    /// medicines.
    pub dosage: Option<Dosage>,

    /// Additional prescribing notes, absent when the structured instructions are
    /// sufficient.
    pub notes: Option<String>,

    /// Hospital physician who issued the prescription.
    pub doctor: EmployeeInfo,
}
