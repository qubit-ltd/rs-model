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
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AggregateRef;

/// A named string payload attached to an aggregate root.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(
    name = "payload_aggregate_key",
    fields(aggregate_ref, key),
    ignore_case(key)
))]
pub struct Payload {
    /// Platform-assigned identifier of this aggregate payload.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

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
