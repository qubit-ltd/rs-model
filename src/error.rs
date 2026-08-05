//! Errors produced by domain-model operations.

use thiserror::Error;

/// Describes a failure encountered while constructing, converting, or validating a model.
///
/// The contained message preserves the model-specific context supplied by the caller.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// A model operation failed with the supplied explanatory message.
    #[error("{0}")]
    Message(String),
}
