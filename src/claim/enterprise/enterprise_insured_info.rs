// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise insured-person information.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::claim::enterprise::EnterpriseHistoryClaimAmount;
use crate::claim::enterprise::EnterpriseInsuredType;
use crate::claim::enterprise::EnterpriseOwnership;

/// An enterprise insured person and their linked employee and claim history.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct EnterpriseInsuredInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Insurance product information.
    #[model(opaque)]
    pub product: Info,

    /// Insured person's name.
    pub name: String,

    /// Insured person's credential number.
    #[redact(level = "secret")]
    pub credential_number: String,

    /// Optional enterprise ownership program.
    pub ownership: Option<EnterpriseOwnership>,

    /// Optional enterprise insured-person classification.
    pub insured_type: Option<EnterpriseInsuredType>,

    /// Optional age.
    pub age: Option<i32>,

    /// Optional relationship to the covered employee.
    pub employee_relation: Option<String>,

    /// Optional employee credential number.
    #[redact(level = "secret")]
    pub employee_credential_number: Option<String>,

    /// Optional employee medical-insurance number.
    #[redact(level = "secret")]
    pub employee_medicare_number: Option<String>,

    /// Optional employee name.
    pub employee_name: Option<String>,

    /// Optional employee company.
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
