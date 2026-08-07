// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Session expiration information.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::ExpiredReason;

/// Timestamp and reason for session expiration.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Expired {
    /// UTC expiration timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub time: Option<DateTime<Utc>>,

    /// Expiration reason.
    #[model(index)]
    pub reason: ExpiredReason,
}
