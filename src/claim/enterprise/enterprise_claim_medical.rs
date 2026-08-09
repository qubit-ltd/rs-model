// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical encounters imported for employer-sponsored claims.

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

/// A treatment encounter imported into an enterprise claim, with its invoices
/// and eligibility attributes.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaimMedical {
    /// Typed identifier used when this enterprise treatment record is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// First date of the treatment episode represented by this record.
    pub treatment_start_date: NaiveDate,

    /// Final date of the treatment episode represented by this record.
    pub treatment_end_date: NaiveDate,

    /// Source encounter sequence number, absent when the import does not
    /// provide one.
    pub number: Option<String>,

    /// Insurer-assigned application identifier, if this encounter was sent to
    /// the insurer.
    pub claim_apply_id: Option<String>,

    /// Medical category used for benefit calculation, absent when unclassified.
    pub medical_category: Option<DictEntryInfo>,

    /// Disease recorded for the treatment episode, absent when not supplied.
    pub disease: Option<DictEntryInfo>,

    /// Treating hospital, absent when the import cannot identify one.
    pub hospital: Option<DictEntryInfo>,

    /// Hospital grade used by the enterprise reimbursement rules, if supplied.
    pub hospital_level: Option<i32>,

    /// Name of the source operator, absent when the import has no operator data.
    pub operator_name: Option<String>,

    /// Enterprise insured-person classification.
    pub insured_type: EnterpriseInsuredType,

    /// Status of saving this imported treatment record.
    pub status: SaveStatus,

    /// Invoice evidence belonging to this treatment episode.
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
