// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! External-standard codes.

use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::mixin::StatefulInfo;

/// A code supplied by an application under an optional external standard.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct Code {
    /// Optional owning application information.
    #[model(opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<StatefulInfo>,

    /// Optional coding standard.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,

    /// Code value.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub code: String,
}

impl Code {
    /// Returns whether all source properties are absent or empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.app.is_none()
            && self.standard.as_ref().is_none_or(String::is_empty)
            && self.code.is_empty()
    }
}

impl Emptyful for Code {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Code {
    fn normalize(&mut self) {
        self.standard.normalize();
        self.code.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
