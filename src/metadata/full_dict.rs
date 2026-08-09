// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Dictionaries with embedded entries.

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
use super::Dict;
use super::DictEntry;
use super::Scope;
use crate::commons::State;

/// A dictionary carrying its complete entry collection.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(name = "dict_code", fields(code), ignore_case(code)))]
pub struct FullDict {
    /// Persisted identifier inherited from `Dict` in the source model.
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

    /// Optional user-facing explanation of the dictionary's intended use.
    pub description: Option<String>,

    /// Optional administrator note that is distinct from the user-facing description.
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

    /// Embedded entries, or `None` when they were not loaded.
    #[model(reference(target = DictEntry, target_field = id, must_exist = false))]
    #[redact(nested)]
    pub entries: Option<Vec<DictEntry>>,
}

impl FullDict {
    /// Translates an entry code using source-compatible cleanup and matching.
    #[must_use]
    pub fn translate(&self, value: &str) -> Option<String> {
        let entries = self.entries.as_deref()?;
        if entries.is_empty() {
            return None;
        }
        let exact = |candidate: &str| {
            entries
                .iter()
                .find(|entry| entry.code.eq_ignore_ascii_case(candidate))
                .map(|entry| entry.name.clone())
                .or_else(|| {
                    entries
                        .iter()
                        .find_map(|entry| entry.match_code_and_format_name(candidate))
                })
        };
        exact(value).or_else(|| {
            let trimmed = value.trim();
            exact(trimmed).or_else(|| {
                trimmed
                    .bytes()
                    .all(|byte| byte.is_ascii_digit())
                    .then(|| trimmed.trim_start_matches('0'))
                    .and_then(exact)
            })
        })
    }

    /// Returns whether all dictionary and entry fields are empty.
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
            && self.entries.as_ref().is_none_or(Vec::is_empty)
    }
}

impl Emptyful for FullDict {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for FullDict {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.standard_doc.normalize();
        self.standard_code.normalize();
        self.url.normalize();
        self.description.normalize();
        self.comment.normalize();
        self.category.normalize();
        self.entries.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}

impl From<Dict> for FullDict {
    fn from(dict: Dict) -> Self {
        Self {
            id: dict.id,
            code: dict.code,
            name: dict.name,
            scope: dict.scope,
            standard_doc: dict.standard_doc,
            standard_code: dict.standard_code,
            url: dict.url,
            description: dict.description,
            comment: dict.comment,
            category: dict.category,
            state: dict.state,
            predefined: dict.predefined,
            create_time: dict.create_time,
            modify_time: dict.modify_time,
            delete_time: dict.delete_time,
            entries: None,
        }
    }
}
