// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Lightweight personally identifying information.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::{Gender, User};
use crate::upload::Attachment;

/// A compact person snapshot used by references outside the person domain.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PersonInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Person's name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional registered ASCII user name.
    #[model(
        reference(target = User, target_field = username),
        index,
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    pub username: Option<String>,

    /// Optional gender.
    pub gender: Option<Gender>,

    /// Optional birthday.
    pub birthday: Option<NaiveDate>,

    /// Optional credential summary.
    #[model(index)]
    #[redact(nested)]
    pub credential: Option<CredentialInfo>,

    /// Optional mobile number.
    #[model(index)]
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Optional ASCII email address.
    #[model(index, text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Optional profile photograph.
    #[model(reference(target = Attachment, target_field = id))]
    pub photo: Option<Attachment>,

    /// Whether this is test data.
    #[model(index)]
    pub test: bool,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
