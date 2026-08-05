// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Dictionaries with embedded entries.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::{Dict, DictEntry};

/// A dictionary DTO carrying its complete entry collection.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullDict {
    /// Dictionary fields inherited by the Java source type.
    #[serde(flatten)]
    #[redact(nested)]
    pub dict: Dict,
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
}
