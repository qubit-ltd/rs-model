// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Metadata ownership-scope classifications.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Identifies the ownership boundary of a metadata record.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScopeType {
    /// System-wide scope.
    #[default]
    System,
    /// Tenant scope.
    Tenant,
    /// Application scope.
    App,
    /// Organization scope.
    Organization,
}
