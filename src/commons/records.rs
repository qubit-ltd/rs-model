//! Shared records used across the migrated model domains.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use super::{CredentialType, VerifyState};
use crate::mixin::StatefulInfo;

/// Identifies the owner of a domain object.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct Owner {
    /// Owning entity name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Owner's persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
}

/// A set of owners represented by their entity and identifier pairs.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct Owners {
    /// Owners in source order.
    pub values: Vec<Owner>,
}

/// Lightweight credential information.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct CredentialInfo {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Credential classification.
    pub r#type: CredentialType,
    /// Credential number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,
    /// Optional verification state.
    pub verified: Option<VerifyState>,
}

/// A credential with ownership and audit fields.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Credential {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Credential owner.
    pub owner: Owner,
    /// Credential classification.
    pub r#type: CredentialType,
    /// Credential number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,
    /// Optional verification state.
    pub verified: Option<VerifyState>,
    /// Position among the owner's credentials.
    pub index: i32,
    /// Optional display title.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,
    /// Optional description.
    pub description: Option<String>,
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

/// A named payload belonging to an owner.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Payload {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Payload key.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub key: String,
    /// Optional payload value.
    pub value: Option<String>,
    /// Payload owner.
    pub owner: Owner,
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

/// Records failures and the last authorization time.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct AuthorizeRecord {
    /// Consecutive authorization failures.
    pub failures: Option<i32>,
    /// Optional last authorization timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub time: Option<DateTime<Utc>>,
}

/// A common category record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct Category {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Entity discriminator.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Globally unique category code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Category name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional icon URL or key.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Query-computed display title.
    pub title: Option<String>,
    /// Optional parent category information.
    #[model(opaque)]
    pub parent: Option<InfoWithEntity>,
    /// Whether this record is predefined.
    pub predefined: bool,
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

/// Source-system metadata for an imported record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct Source {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Importing application information.
    pub app: StatefulInfo,
    /// Entity discriminator.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Source code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Source name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional source description.
    pub description: Option<String>,
    /// Optional category information.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,
    /// Optional provider application information.
    pub provider_app: Option<StatefulInfo>,
    /// Optional provider organization information.
    pub provider_organization: Option<StatefulInfo>,
    /// Whether this source is predefined.
    pub predefined: bool,
    /// Optional UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
