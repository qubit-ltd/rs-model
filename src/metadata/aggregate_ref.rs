// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Aggregate-root references.

use qubit_mixin::{Emptyful, Normalizable};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Identifies an aggregate root and an optional property on it.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(key(name = "aggregate_ref", fields(type, id, property)))]
pub struct AggregateRef {
    /// Aggregate-root entity type.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub r#type: String,
    /// Aggregate-root identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Optional property name within the aggregate root.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
}

impl Emptyful for AggregateRef {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for AggregateRef {
    fn normalize(&mut self) {
        self.r#type.normalize();
        self.property.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}

impl AggregateRef {
    /// Returns whether this reference contains no aggregate identity.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.r#type.is_empty() && self.id.is_none() && self.property.is_none()
    }
}
