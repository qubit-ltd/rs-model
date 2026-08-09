// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Full patient records used in clinical prescription content.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::commons::State;
use crate::contact::Phone;
use crate::person::Gender;
use crate::person::PersonInfo;

/// Clinical identity, coverage, and contact details for a patient receiving care.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Patient {
    /// Typed identifier used when this patient record is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Linked registered-user identifier, absent when the patient has no user
    /// account.
    #[model(opaque)]
    pub user_id: Id,

    /// Linked full person-record identifier, absent when no person record is
    /// associated.
    #[model(opaque)]
    pub person_id: Id,

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

    /// Verified identity credential, absent when it has not been collected.
    pub credential: Option<CredentialInfo>,

    /// Patient's mobile telephone number.
    pub mobile: Phone,

    /// Contact email address, absent when the patient has not supplied one.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Guardian or legal representative, absent for an unaccompanied adult.
    pub guardian: Option<PersonInfo>,

    /// Insurance-coverage indicator, absent when eligibility has not been
    /// determined.
    pub has_medicare: Option<bool>,

    /// Medical-insurance card credential, absent when no card was presented.
    pub medicare_card: Option<CredentialInfo>,

    /// City administering the patient's coverage, absent when not applicable.
    #[model(opaque)]
    pub medicare_city: Option<Info>,

    /// Administrative or clinical note, absent when no extra context is needed.
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
