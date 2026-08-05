//! Lightweight system-user information values.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::State;
use crate::contact::Phone;
use crate::person::Gender;

/// A compact user-information snapshot.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct UserInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique ASCII user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: String,
    /// Optional display name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub name: Option<String>,
    /// Optional nickname.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub nickname: Option<String>,
    /// Optional gender.
    pub gender: Option<Gender>,
    /// Optional ASCII avatar URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub avatar: Option<String>,
    /// Optional mobile number.
    pub mobile: Option<Phone>,
    /// Optional email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,
    /// Lifecycle state.
    pub state: State,
    /// Whether this is a test user.
    pub test: bool,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
