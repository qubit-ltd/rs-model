// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact operation-log projections.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::Action;

/// Compact information derived from a full operation log.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct OperationLogInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Audited action.
    pub action: Action,

    /// Optional resource name.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    /// Optional resource property.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,

    /// Optional username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Optional application name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,

    /// Client IP address.
    #[redact(level = "secret")]
    pub client_ip: String,

    /// Optional operation outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Optional error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    /// Optional error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// Optional UTC request timestamp.
    #[model(time(precision = millisecond, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}
