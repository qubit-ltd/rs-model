// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Distributed-system hosts.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// A provider host identified by its device identifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(unique(name = "host_udid", fields(udid)))]
pub struct Host {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Provider name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub provider: String,
    /// Exact case-sensitive device identifier.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub udid: String,
}
