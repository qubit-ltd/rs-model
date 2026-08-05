// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors produced by domain-model operations.

/// Identifies one model-field constraint violation without retaining its
/// rejected value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationViolation {
    /// The field whose constraint failed.
    pub field: String,
    /// The constraint failure reason.
    pub reason: String,
}
