// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Aggregate payload values.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::AggregateRef;

/// A named string payload attached to an aggregate root.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[model(unique(name = "payload_aggregate_key", fields(aggregate_ref, key)))]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Payload key unique within the aggregate.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub key: String,
    /// Optional payload value.
    #[model(text(min_chars = 1, max_chars = 256))]
    #[redact(level = "secret")]
    pub value: Option<String>,
    /// Owning aggregate reference.
    #[redact(nested)]
    pub aggregate_ref: Option<AggregateRef>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Payload {
    /// Creates a payload with its key and optional value.
    #[must_use]
    pub fn new(key: impl Into<String>, value: Option<String>) -> Self {
        Self {
            key: key.into(),
            value,
            ..Self::default()
        }
    }

    /// Returns whether the payload key is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
    }
}
