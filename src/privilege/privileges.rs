//! Permission collection and its comma-separated wire codec.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Ordered privilege names assigned to a role or session.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Privileges(pub Vec<String>);

/// Errors produced when decoding a permission list.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PrivilegesCodecError {
    /// A list contains a privilege name that is empty after trimming.
    #[error("privilege at index {index} is empty")]
    EmptyPrivilege {
        /// Zero-based element index.
        index: usize,
    },
    /// A privilege contains the comma delimiter and cannot be encoded unambiguously.
    #[error("privilege at index {index} contains a comma")]
    ContainsSeparator {
        /// Zero-based element index.
        index: usize,
    },
}

impl Privileges {
    /// Decodes the Java-compatible comma-separated representation.
    ///
    /// `None` remains `None`; an empty string becomes an empty collection. Surrounding
    /// whitespace and empty segments are ignored, matching the source `Splitter` settings.
    pub fn decode(value: Option<&str>) -> Result<Option<Self>, PrivilegesCodecError> {
        let Some(value) = value else {
            return Ok(None);
        };
        if value.is_empty() {
            return Ok(Some(Self::default()));
        }
        let values: Vec<String> = value
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        Self::validate(&values)?;
        Ok(Some(Self(values)))
    }

    /// Encodes this collection using the Java-compatible comma delimiter.
    ///
    /// Returns an error if a member is empty or contains a comma, because either value would
    /// lose information in the encoded form.
    pub fn encode(&self) -> Result<String, PrivilegesCodecError> {
        Self::validate(&self.0)?;
        Ok(self.0.join(","))
    }

    /// Ensures privilege names can round-trip through the comma codec.
    fn validate(values: &[String]) -> Result<(), PrivilegesCodecError> {
        for (index, value) in values.iter().enumerate() {
            if value.trim().is_empty() {
                return Err(PrivilegesCodecError::EmptyPrivilege { index });
            }
            if value.contains(',') {
                return Err(PrivilegesCodecError::ContainsSeparator { index });
            }
        }
        Ok(())
    }
}
