// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Aggregate payload values.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AggregateRef;

/// A named string payload attached to an aggregate root.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
#[model(unique(
    name = "payload_aggregate_key",
    fields(aggregate_ref, key),
    ignore_case(key)
))]
pub struct Payload {
    /// Persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Payload key unique within the aggregate.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub key: String,

    /// Optional payload value.
    #[model(text(min_chars = 1, max_chars = 256))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Owning aggregate reference.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_ref: Option<AggregateRef>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Emptyful for Payload {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Payload {}

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
