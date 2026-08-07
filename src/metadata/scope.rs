// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Metadata ownership scopes.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::ScopeType;

/// A scope classification and its optional owner identifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct Scope {
    /// Scope classification.
    #[model(index)]
    pub r#type: ScopeType,

    /// Tenant or application identifier for a non-system scope.
    #[model(index)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
