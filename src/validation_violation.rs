// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Field-level validation failures reported while accepting domain-model data.

/// Describes one failed constraint for a model field.
///
/// A [`crate::ModelError::ValidationFailed`] can contain several violations;
/// each one names the affected field and explains the violated constraint.
/// Callers must ensure that `field` and `reason` do not contain sensitive
/// rejected values before constructing this value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationViolation {
    /// The caller-supplied field name whose constraint was not satisfied.
    pub field: String,

    /// The caller-supplied explanation of the failed constraint, stored verbatim.
    pub reason: String,
}
