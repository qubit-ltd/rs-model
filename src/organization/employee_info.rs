// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight employee information used by cross-domain references.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::commons::State;
use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::person::Gender;
use crate::person::User;
use crate::upload::Attachment;

/// Represents an employee snapshot including organization and department references.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(
    unique(name = "employee_info_code", fields(code), ignore_case(code)),
    unique(
        name = "employee_info_organization_internal_code",
        fields(organization, internal_code),
        ignore_case(internal_code)
    ),
    unique(name = "employee_info_credential", fields(credential)),
    unique(name = "employee_info_mobile", fields(mobile))
)]
pub struct EmployeeInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII employee code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Optional ASCII internal organization code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: Option<String>,

    /// Optional registered ASCII user name.
    #[model(
        reference(target = User, target_field = username),
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    pub username: Option<String>,

    /// Employee name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Employee gender.
    pub gender: Gender,

    /// Optional birthday.
    pub birthday: Option<NaiveDate>,

    /// Optional credential summary.
    pub credential: Option<CredentialInfo>,

    /// Employee mobile number.
    pub mobile: Phone,

    /// Employer information.
    pub organization: StatefulInfo,

    /// Optional department information.
    pub department: Option<StatefulInfo>,

    /// Optional profile photo.
    #[model(reference(target = Attachment, target_field = id, must_exist = false))]
    pub photo: Option<Attachment>,

    /// Employment lifecycle state.
    pub state: State,

    /// Whether this is test data.
    pub test: bool,

    /// Optional UTC deletion timestamp.
    pub delete_time: Option<DateTime<Utc>>,
}
