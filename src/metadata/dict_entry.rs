// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Data-dictionary entries and parameter matching.

use chrono::{DateTime, Utc};
use qubit_mixin::{Emptyful, Normalizable};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

use crate::mixin::StatefulInfo;

use super::{Dict, DictEntryInfo};

/// An entry belonging to a data dictionary.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(unique(name = "dict_entry_dict_code", fields(dict, code), ignore_case(code)))]
pub struct DictEntry {
    /// Persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Owning dictionary information.
    #[model(reference(target = Dict, target_field = info))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict: Option<StatefulInfo>,
    /// Case-insensitive code unique within the dictionary.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Display-name template.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Optional parent-entry information.
    #[model(reference(target = DictEntry, target_field = info))]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<DictEntryInfo>,
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

impl DictEntry {
    /// Creates an entry with a code and display-name template.
    #[must_use]
    pub fn new(code: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            ..Self::default()
        }
    }

    /// Returns this entry's compact information projection.
    #[must_use]
    pub fn info(&self) -> DictEntryInfo {
        DictEntryInfo {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            dict_id: self.dict.as_ref().and_then(|dict| dict.id),
            params: None,
            delete_time: self.delete_time,
        }
    }

    /// Assigns the fields carried by a compact entry projection.
    pub fn assign_info(&mut self, info: &DictEntryInfo) {
        self.id = info.id;
        self.code.clone_from(&info.code);
        self.name.clone_from(&info.name);
        if let Some(dict_id) = info.dict_id {
            self.dict.get_or_insert_with(StatefulInfo::default).id = Some(dict_id);
        }
        self.delete_time = info.delete_time;
    }

    /// Returns the code with numbered placeholders substituted.
    #[must_use]
    pub fn display_code(&self, params: &[&str]) -> String {
        format_with_params(&self.code, params)
    }

    /// Returns the name with numbered placeholders substituted.
    #[must_use]
    pub fn display_name(&self, params: &[&str]) -> String {
        format_with_params(&self.name, params)
    }

    /// Returns whether the code contains a numbered placeholder.
    #[must_use]
    pub fn has_parameter(&self) -> bool {
        placeholder_ranges(&self.code).next().is_some()
    }

    /// Matches a concrete code and formats the name with captured parameters.
    #[must_use]
    pub fn match_code_and_format_name(&self, value: &str) -> Option<String> {
        let ranges: Vec<_> = placeholder_ranges(&self.code).collect();
        if ranges.is_empty() {
            return self
                .code
                .eq_ignore_ascii_case(value)
                .then(|| self.name.clone());
        }
        let mut pattern = String::from("^");
        let mut previous_end = 0;
        for (start, end) in ranges {
            pattern.push_str(&regex::escape(&self.code[previous_end..start]));
            pattern.push_str(r"([\p{L}\p{N}]+)");
            previous_end = end;
        }
        pattern.push_str(&regex::escape(&self.code[previous_end..]));
        pattern.push('$');
        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(true)
            .build()
            .ok()?;
        let captures = regex.captures(value)?;
        let params: Vec<_> = captures
            .iter()
            .skip(1)
            .filter_map(|capture| capture.map(|value| value.as_str()))
            .collect();
        Some(format_with_params(&self.name, &params))
    }

    /// Returns whether all identifying fields are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.dict.as_ref().is_none_or(stateful_info_is_empty)
            && self.code.is_empty()
            && self.name.is_empty()
            && self.description.as_ref().is_none_or(String::is_empty)
            && self.comment.as_ref().is_none_or(String::is_empty)
            && self.parent.as_ref().is_none_or(DictEntryInfo::is_empty)
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for DictEntry {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for DictEntry {
    fn normalize(&mut self) {
        if let Some(dict) = &mut self.dict {
            dict.code.normalize();
            dict.name.normalize();
            if stateful_info_is_empty(dict) {
                self.dict = None;
            }
        }
        self.code.normalize();
        self.name.normalize();
        self.description.normalize();
        self.comment.normalize();
        self.parent.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Reports whether a stateful projection carries no source value.
fn stateful_info_is_empty(info: &StatefulInfo) -> bool {
    info.id.is_none()
        && info.code.is_empty()
        && info.name.is_empty()
        && info.state.is_none()
        && info.delete_time.is_none()
}

/// Replaces numbered placeholders in `template` with the supplied parameters.
pub(super) fn format_with_params<T: AsRef<str>>(template: &str, params: &[T]) -> String {
    params
        .iter()
        .enumerate()
        .fold(template.to_owned(), |result, (index, value)| {
            result.replace(&format!("{{{index}}}"), value.as_ref())
        })
}

/// Locates well-formed numbered placeholders within a template.
fn placeholder_ranges(value: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    value.match_indices('{').filter_map(|(start, _)| {
        let suffix = &value[start + 1..];
        let close = suffix.find('}')?;
        let digits = &suffix[..close];
        (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
            .then_some((start, start + close + 2))
    })
}
