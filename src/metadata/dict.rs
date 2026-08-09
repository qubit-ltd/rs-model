// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Data-dictionary definitions.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Category;
use super::Scope;
use crate::commons::State;

/// A data dictionary without its entry collection.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(name = "dict_code", fields(code), ignore_case(code)))]
pub struct Dict {
    /// Platform-assigned identifier of this dictionary definition.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique stable code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Dictionary name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Ownership scope.
    #[model(index)]
    pub scope: Option<Scope>,

    /// Optional governing standards document.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub standard_doc: Option<String>,

    /// Optional code in the governing standard.
    #[model(index, text(min_chars = 1, max_chars = 64))]
    pub standard_code: Option<String>,

    /// Optional documentation URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub url: Option<String>,

    /// Optional user-facing explanation of the dictionary's purpose.
    pub description: Option<String>,

    /// Optional administrator note kept separate from the user-facing description.
    pub comment: Option<String>,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Lifecycle state.
    #[model(index)]
    #[serde(default)]
    pub state: State,

    /// Whether the dictionary is predefined.
    #[model(index)]
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Dict {
    /// Returns whether the dictionary has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == Id::default()
            && self.code.is_empty()
            && self.name.is_empty()
            && self.scope.is_none()
            && self.standard_doc.as_ref().is_none_or(String::is_empty)
            && self.standard_code.as_ref().is_none_or(String::is_empty)
            && self.url.as_ref().is_none_or(String::is_empty)
            && self.description.as_ref().is_none_or(String::is_empty)
            && self.comment.as_ref().is_none_or(String::is_empty)
            && self.category.as_ref().is_none_or(Emptyful::is_empty)
            && self.state == State::Normal
            && !self.predefined
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for Dict {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Dict {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.standard_doc.normalize();
        self.standard_code.normalize();
        self.url.normalize();
        self.description.normalize();
        self.comment.normalize();
        self.category.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
