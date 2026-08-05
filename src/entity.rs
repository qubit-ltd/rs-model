//! Entity classifications used by shared model references.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies a domain entity category.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Entity {
    /// Application entity.
    App,
    /// Category entity.
    Category,
    /// Credential entity.
    Credential,
    /// Dictionary entity.
    Dict,
    /// Dictionary entry entity.
    DictEntry,
    /// Source entity.
    Source,
    /// Payload entity.
    Payload,
    /// Session entity.
    Session,
    /// Verification-code entity.
    VerifyCode,
}

impl Entity {
    /// Returns the stable lowercase entity identifier.
    ///
    /// # Returns
    /// The identifier used for serialization-independent entity matching.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Category => "category",
            Self::Credential => "credential",
            Self::Dict => "dict",
            Self::DictEntry => "dict_entry",
            Self::Source => "source",
            Self::Payload => "payload",
            Self::Session => "session",
            Self::VerifyCode => "verify_code",
        }
    }
}
