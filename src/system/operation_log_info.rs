// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compact operation-log projections.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Action;

/// Compact information derived from a full operation log.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct OperationLogInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Audited action.
    pub action: Action,

    /// Optional resource name.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub resource: Option<String>,

    /// Optional resource property.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub property: Option<String>,

    /// Optional username.
    pub username: Option<String>,

    /// Optional application name.
    pub app: Option<String>,

    /// Client IP address.
    #[redact(level = "secret")]
    pub client_ip: String,

    /// Optional operation outcome.
    pub success: Option<bool>,

    /// Optional error code.
    pub error_code: Option<String>,

    /// Optional error message.
    pub error_message: Option<String>,

    /// Optional UTC request timestamp.
    #[model(time(precision = millisecond, normalization = utc))]
    pub timestamp: Option<DateTime<Utc>>,
}
