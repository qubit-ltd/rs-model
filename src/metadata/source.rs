// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Channel and acquisition sources.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Emptyful;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Category;
use super::Scope;

/// A channel source associated with an entity type.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
#[model(unique(name = "source_code", fields(code), ignore_case(code)))]
pub struct Source {
    /// Persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Globally unique ASCII code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Source name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Ownership scope.
    #[model(index)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,

    /// Associated entity name.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<InfoWithEntity>,

    /// Whether this source is predefined.
    #[model(index)]
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Source {
    /// Returns whether the source has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.code.is_empty()
            && self.name.is_empty()
            && self.scope.is_none()
            && self.entity.is_empty()
            && self.description.as_ref().is_none_or(String::is_empty)
            && self.category.as_ref().is_none_or(Emptyful::is_empty)
            && !self.predefined
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for Source {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Source {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.entity.normalize();
        self.description.normalize();
        self.category.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
