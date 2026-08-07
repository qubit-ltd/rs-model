// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Task lifecycle vocabularies and errors.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// An operation applied to a task.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAction {
    /// The Submit classification.
    Submit,
    /// The Init classification.
    Init,
    /// The Start classification.
    Start,
    /// The Cancel classification.
    Cancel,
    /// The Fail classification.
    Fail,
    /// The Success classification.
    Success,
}
