// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Metadata ownership scopes.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::ScopeType;

/// A scope classification and its optional owner identifier.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Scope {
    /// Scope classification.
    #[model(index)]
    pub r#type: ScopeType,

    /// Tenant or application identifier for a non-system scope.
    #[model(index, opaque)]
    pub id: Id,
}
