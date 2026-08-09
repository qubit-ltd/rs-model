// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Distributed-system hosts.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Represents a provider host identified by its device identifier.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Host {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Provider name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub provider: String,

    /// Exact case-sensitive device identifier.
    #[model(unique, text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub udid: String,
}
