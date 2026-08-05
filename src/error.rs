//! Errors produced by domain-model operations.

use thiserror::Error;

/// Describes a failure encountered while constructing, converting, or validating a model.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// A named model field violates a declared validation constraint.
    ///
    /// `field` identifies the invalid field and `reason` identifies the failed
    /// constraint without including the rejected field value.
    #[error("model validation failed for field `{field}`: {reason}")]
    ValidationFailed {
        /// The field whose constraint failed.
        field: String,
        /// The constraint failure reason.
        reason: String,
    },
}
