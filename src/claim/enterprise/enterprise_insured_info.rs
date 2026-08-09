// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Covered-person data imported from an enterprise insurance program.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::claim::enterprise::EnterpriseHistoryClaimAmount;
use crate::claim::enterprise::EnterpriseInsuredType;
use crate::claim::enterprise::EnterpriseOwnership;

/// A covered person with employee linkage and historical enterprise-claim
/// amounts used for eligibility and calculation.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct EnterpriseInsuredInfo {
    /// Typed identifier used when this enterprise insured-person record is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Insurance product information.
    #[model(opaque)]
    pub product: Info,

    /// Insured person's name.
    pub name: String,

    /// Insured person's credential number.
    #[redact(level = "secret")]
    pub credential_number: String,

    /// Enterprise ownership program, absent when the import cannot classify it.
    pub ownership: Option<EnterpriseOwnership>,

    /// Covered-person category, absent when the enterprise source omits it.
    pub insured_type: Option<EnterpriseInsuredType>,

    /// Age used by enterprise eligibility rules, absent when not supplied.
    pub age: Option<i32>,

    /// Relationship to the covered employee, absent for the employee themself or
    /// when the source omits it.
    pub employee_relation: Option<String>,

    /// Employee credential number, absent when no employee linkage is supplied.
    #[redact(level = "secret")]
    pub employee_credential_number: Option<String>,

    /// Employee medical-insurance number, absent when the source lacks it.
    #[redact(level = "secret")]
    pub employee_medicare_number: Option<String>,

    /// Linked employee name, absent when the covered person has no supplied link.
    pub employee_name: Option<String>,

    /// Linked employee's company, absent when the source does not identify one.
    pub employee_company: Option<String>,

    /// Historical claim amounts.
    pub claim_amounts: Vec<EnterpriseHistoryClaimAmount>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
