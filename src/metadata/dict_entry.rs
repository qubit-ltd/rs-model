// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Data-dictionary entries and parameter matching.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

use crate::mixin::StatefulInfo;

use super::{Dict, DictEntryInfo};

/// An entry belonging to a data dictionary.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(unique(name = "dict_entry_dict_code", fields(dict, code)))]
#[serde(rename_all = "camelCase")]
pub struct DictEntry {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Owning dictionary information.
    #[model(reference(target = Dict, target_field = info))]
    pub dict: Option<StatefulInfo>,
    /// Case-insensitive code unique within the dictionary.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Display-name template.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional comment.
    pub comment: Option<String>,
    /// Optional parent-entry information.
    #[model(reference(target = DictEntry, target_field = info))]
    #[redact(nested)]
    pub parent: Option<DictEntryInfo>,
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
            params: Vec::new(),
            delete_time: self.delete_time,
        }
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
            return None;
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
        self.id.is_none() && self.code.is_empty() && self.name.is_empty()
    }
}

pub(super) fn format_with_params<T: AsRef<str>>(template: &str, params: &[T]) -> String {
    params
        .iter()
        .enumerate()
        .fold(template.to_owned(), |result, (index, value)| {
            result.replace(&format!("{{{index}}}"), value.as_ref())
        })
}

fn placeholder_ranges(value: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    value.match_indices('{').filter_map(|(start, _)| {
        let suffix = &value[start + 1..];
        let close = suffix.find('}')?;
        let digits = &suffix[..close];
        (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
            .then_some((start, start + close + 2))
    })
}
