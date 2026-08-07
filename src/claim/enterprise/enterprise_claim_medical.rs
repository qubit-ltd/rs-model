// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim medical encounters.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::enterprise::EnterpriseClaimInvoice;
use crate::claim::enterprise::EnterpriseInsuredType;
use crate::claim::enterprise::SaveStatus;
use crate::commons::DictEntryInfo;

/// A medical encounter imported for an enterprise insurance claim.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaimMedical {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Treatment start date.
    pub treatment_start_date: NaiveDate,

    /// Treatment end date.
    pub treatment_end_date: NaiveDate,

    /// Optional medical encounter sequence number.
    pub number: Option<String>,

    /// Optional insurer-side claim application identifier.
    pub claim_apply_id: Option<String>,

    /// Optional medical-category dictionary entry.
    pub medical_category: Option<DictEntryInfo>,

    /// Optional disease dictionary entry.
    pub disease: Option<DictEntryInfo>,

    /// Optional hospital dictionary entry.
    pub hospital: Option<DictEntryInfo>,

    /// Optional hospital level.
    pub hospital_level: Option<i32>,

    /// Optional operator name.
    pub operator_name: Option<String>,

    /// Enterprise insured-person classification.
    pub insured_type: EnterpriseInsuredType,

    /// Import state.
    pub status: SaveStatus,

    /// Invoices belonging to this encounter.
    pub invoices: Vec<EnterpriseClaimInvoice>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
