// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Metadata ownership scopes.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::ScopeType;

/// A scope classification and its optional owner identifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Scope classification.
    pub scope_type: ScopeType,
    /// Tenant or application identifier for a non-system scope.
    pub id: Option<i64>,
}
