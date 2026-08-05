// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Complete patient records.

use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commons::{
        CredentialInfo,
        State,
    },
    contact::Phone,
    person::{
        Gender,
        PersonInfo,
    },
};

/// A hospital patient with identity, insurance, contact, and lifecycle data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Patient {
    /// Optional persisted patient identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Optional registered-user identifier.
    pub user_id: Option<i64>,
    /// Optional complete personal-record identifier.
    pub person_id: Option<i64>,
    /// Owning hospital information.
    #[model(opaque)]
    pub hospital: Info,
    /// Globally unique patient code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Patient code within the owning hospital.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: String,
    /// Patient's legal name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Patient's gender.
    pub gender: Gender,
    /// Patient's date of birth.
    pub birthday: NaiveDate,
    /// Optional verified identity credential.
    pub credential: Option<CredentialInfo>,
    /// Patient's mobile telephone number.
    pub mobile: Phone,
    /// Optional email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,
    /// Optional guardian information.
    pub guardian: Option<PersonInfo>,
    /// Optional indication that the patient has medical insurance.
    pub has_medicare: Option<bool>,
    /// Optional medical-insurance credential.
    pub medicare_card: Option<CredentialInfo>,
    /// Optional medical-insurance city.
    #[model(opaque)]
    pub medicare_city: Option<Info>,
    /// Optional remark.
    pub comment: Option<String>,
    /// Current lifecycle state.
    pub state: State,
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
