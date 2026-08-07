// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight employee information used by cross-domain references.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::{CredentialInfo, State};
use crate::contact::Phone;
use crate::mixin::StatefulInfo;
use crate::person::Gender;
use crate::upload::Attachment;

/// An employee snapshot including organization and department references.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct EmployeeInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Globally unique ASCII employee code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Optional ASCII internal organization code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: Option<String>,

    /// Optional registered ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
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
    pub photo: Option<Attachment>,

    /// Employment lifecycle state.
    pub state: State,

    /// Whether this is test data.
    pub test: bool,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
