// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact legacy system logs.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::ErrorInfo;
use crate::person::UserInfo;

/// A compact operation outcome ordered by its timestamp.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
#[model(unique(name = "log_id", fields(id)))]
pub struct Log {
    /// Optional unique log identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Optional UTC event timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// Optional operator information.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<UserInfo>,

    /// Optional source IP address.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Operation name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub operation: String,

    /// Optional target entity type.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,

    /// Optional target identifier.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,

    /// Optional operation outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Optional structured error.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}
