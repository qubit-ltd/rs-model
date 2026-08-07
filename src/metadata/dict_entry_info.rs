// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact dictionary-entry information.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Dict;
use super::dict_entry::format_with_params;

/// Compact information for a dictionary entry.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact)]
#[serde(default)]
pub struct DictEntryInfo {
    /// Persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Entry code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Entry name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Owning dictionary identifier.
    #[model(reference(target = Dict, target_field = id))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<i64>,

    /// Values substituted into numbered placeholders.
    #[model(sequence(min_items = 1, max_items = 5))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<String>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl DictEntryInfo {
    /// Creates an info value unless every supplied identity field is absent.
    #[must_use]
    pub fn create(
        id: Option<i64>,
        code: Option<&str>,
        name: Option<&str>,
    ) -> Option<Self> {
        if id.is_none() && code.is_none() && name.is_none() {
            None
        } else {
            Some(Self {
                id,
                code: code.unwrap_or_default().to_owned(),
                name: name.unwrap_or_default().to_owned(),
                ..Self::default()
            })
        }
    }

    /// Returns the code with numbered placeholders substituted.
    #[must_use]
    pub fn display_code(&self) -> String {
        self.params.as_deref().map_or_else(
            || self.code.clone(),
            |params| format_with_params(&self.code, params),
        )
    }

    /// Returns the name with numbered placeholders substituted.
    #[must_use]
    pub fn display_name(&self) -> String {
        self.params.as_deref().map_or_else(
            || self.name.clone(),
            |params| format_with_params(&self.name, params),
        )
    }

    /// Returns whether all identifying fields are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.code.is_empty()
            && self.name.is_empty()
            && self.dict_id.is_none()
            && self.params.as_ref().is_none_or(Vec::is_empty)
            && self.delete_time.is_none()
    }
}

impl Emptyful for DictEntryInfo {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for DictEntryInfo {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.params.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Serialize for DictEntryInfo {
    /// Serializes source fields and Jackson-visible computed display values.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DictEntryInfo", 8)?;
        if let Some(id) = self.id {
            state.serialize_field("id", &id)?;
        }
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        if let Some(dict_id) = self.dict_id {
            state.serialize_field("dict_id", &dict_id)?;
        }
        if let Some(params) = &self.params {
            state.serialize_field("params", params)?;
        }
        if let Some(delete_time) = self.delete_time {
            state.serialize_field("delete_time", &delete_time)?;
        }
        state.serialize_field("display_code", &self.display_code())?;
        state.serialize_field("display_name", &self.display_name())?;
        state.end()
    }
}
