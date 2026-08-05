// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Aggregate-root references.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Identifies an aggregate root and an optional property on it.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateRef {
    /// Aggregate-root entity type.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity_type: String,
    /// Aggregate-root identifier.
    pub id: Option<i64>,
    /// Optional property name within the aggregate root.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub property: Option<String>,
}

impl AggregateRef {
    /// Returns whether this reference contains no aggregate identity.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entity_type.is_empty() && self.id.is_none() && self.property.is_none()
    }
}
