// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Error values emitted by model validation and conversion.

#[path = "error_type.rs"]
mod error_type;
#[path = "model_error.rs"]
mod model_error;
#[path = "validation_violation.rs"]
mod validation_violation;

pub use error_type::ErrorType;
pub use model_error::ModelError;
pub use validation_violation::ValidationViolation;
