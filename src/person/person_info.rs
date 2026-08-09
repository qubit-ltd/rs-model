// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Lightweight personally identifying information.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::Gender;
use crate::person::User;
use crate::upload::Attachment;

/// A compact person snapshot used by references outside the person domain.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct PersonInfo {
    /// Database identifier of the referenced person; default denotes an unsaved person.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Person's name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Account username associated with the person, when the person has one.
    #[model(
        reference(target = User, target_field = username),
        index,
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    pub username: Option<String>,

    /// Gender included for compact identity and demographic displays.
    pub gender: Option<Gender>,

    /// Birth date included when age or identity context is required.
    pub birthday: Option<NaiveDate>,

    /// Redacted identity-credential summary used to distinguish people safely.
    #[model(index)]
    #[redact(nested)]
    pub credential: Option<CredentialInfo>,

    /// Mobile contact channel included in this compact person projection.
    #[model(index)]
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Email contact channel included in this compact person projection.
    #[model(index, text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Profile image used to help staff recognize the person.
    #[model(reference(target = Attachment, target_field = id))]
    pub photo: Option<Attachment>,

    /// Marks synthetic person data excluded from live workflows.
    #[model(index)]
    pub test: bool,

    /// Soft-deletion time copied from the source person; absence means active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
