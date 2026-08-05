// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Signable prescription content.

use chrono::NaiveDateTime;
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    commons::DictEntryInfo,
    medical::{Diagnosis, Dosage, MedicalType, Patient, PrescriptionItem},
    organization::EmployeeInfo,
};

/// The stable clinical content signed and carried by a prescription.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct PrescriptionContent {
    /// Prescription sequence number within the issuing organization.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,
    /// Drug-category dictionary entry.
    pub category: DictEntryInfo,
    /// Prescription-type dictionary entry.
    pub r#type: DictEntryInfo,
    /// Prescription-destination dictionary entry.
    pub direction: DictEntryInfo,
    /// Cost-source dictionary entry.
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
    /// Optional inpatient ward.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub ward: Option<String>,
    /// Optional inpatient bed number.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub bed: Option<String>,
    /// Patient receiving the prescription.
    pub patient: Patient,
    /// Patient's chief complaint.
    pub complaint: String,
    /// Ranked diagnoses.
    #[model(sequence(min_items = 1, max_items = 8))]
    pub diagnoses: Vec<Diagnosis>,
    /// Optional patient weight in kilograms.
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
    /// Optional shared dosage instructions for a herbal prescription.
    pub dosage: Option<Dosage>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Hospital physician who issued the prescription.
    pub doctor: EmployeeInfo,
}
