//! Lightweight personally identifying information.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::Gender;
use crate::upload::Attachment;

/// A compact person snapshot used by references outside the person domain.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PersonInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Person's name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional registered ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: Option<String>,
    /// Optional gender.
    pub gender: Option<Gender>,
    /// Optional birthday.
    pub birthday: Option<NaiveDate>,
    /// Optional credential summary.
    pub credential: Option<CredentialInfo>,
    /// Optional mobile number.
    pub mobile: Option<Phone>,
    /// Optional ASCII email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,
    /// Optional profile photograph.
    pub photo: Option<Attachment>,
    /// Whether this is test data.
    pub test: bool,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
