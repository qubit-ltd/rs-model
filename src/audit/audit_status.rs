// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Audit lifecycle classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Represents the lifecycle state of an audit request.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditStatus {
    /// The audit request was submitted for review.
    Submitted,
}
