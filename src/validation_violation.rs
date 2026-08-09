// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Field-level validation failures reported while accepting domain-model data.

/// Describes one failed constraint without retaining the rejected value.
///
/// A [`crate::ModelError::ValidationFailed`] can contain several violations;
/// each one names the affected field and explains the violated constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationViolation {
    /// The canonical field name whose constraint was not satisfied.
    pub field: String,

    /// A safe explanation of the failed constraint, without the rejected value.
    pub reason: String,
}
