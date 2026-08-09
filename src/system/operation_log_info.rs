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
    /// Database identifier of the source audit entry; default denotes an unsaved projection.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Business action recorded by the audit trail.
    pub action: Action,

    /// Resource namespace or type targeted by the audited action.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub resource: Option<String>,

    /// Resource property or operation detail associated with the audited action.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub property: Option<String>,

    /// Username of the actor when the request was authenticated as a user.
    pub username: Option<String>,

    /// Application through which the action was performed, when known.
    pub app: Option<String>,

    /// Network address observed for the audit event's client.
    #[redact(level = "secret")]
    pub client_ip: String,

    /// Whether the audited action completed successfully; absent when not yet known.
    pub success: Option<bool>,

    /// Structured failure code captured when the audited action did not succeed.
    pub error_code: Option<String>,

    /// Display-safe failure detail captured with the audit event.
    pub error_message: Option<String>,

    /// UTC instant when the audited request began.
    #[model(time(precision = millisecond, normalization = utc))]
    pub timestamp: Option<DateTime<Utc>>,
}
