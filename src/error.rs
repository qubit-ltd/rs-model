// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors at the boundary where external data becomes a domain model.

#[path = "error_type.rs"]
mod error_type;
#[path = "model_error.rs"]
mod model_error;
#[path = "validation_violation.rs"]
mod validation_violation;

/// Broad category assigned to a platform error.
pub use error_type::ErrorType;
/// Failure raised while constructing, converting, or validating a model.
pub use model_error::ModelError;
/// One field-level constraint failure, excluding the rejected value.
pub use validation_violation::ValidationViolation;
