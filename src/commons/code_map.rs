// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External-to-platform code mappings.

use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::{
    Emptyful,
    Normalizable,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use super::Code;

/// Maps a source-system code to a platform code for one entity type.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct CodeMap {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Entity type owning the mapping.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub entity: String,
    /// Source-system code.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Code>,
    /// Platform code value.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub platform_code: String,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl CodeMap {
    /// Returns whether every source property is absent or empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.entity.is_empty()
            && self.source.as_ref().is_none_or(Code::is_empty)
            && self.platform_code.is_empty()
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for CodeMap {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for CodeMap {
    fn normalize(&mut self) {
        self.entity.normalize();
        self.source.normalize();
        self.platform_code.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
