// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External-to-platform code mappings.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Code;

/// Maps a source-system code to a platform code for one entity type.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct CodeMap {
    /// Platform-assigned identifier of this external-to-platform code mapping.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Entity type owning the mapping.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub entity: String,

    /// Source-system code.
    #[redact(nested)]
    pub source: Option<Code>,

    /// Platform code value.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub platform_code: String,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl CodeMap {
    /// Returns whether every source property is absent or empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == Id::default()
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
