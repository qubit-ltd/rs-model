// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Failed message-queue tasks.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::MqType;

/// A message-queue task retained after processing failure.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct MqFailedTask {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Message topic.
    pub topic: String,

    /// Message tag.
    pub tag: String,

    /// Message operation type.
    pub r#type: MqType,

    /// Provider-assigned message identifier.
    pub message_id: String,

    /// Business message key.
    pub message_key: String,

    /// Serialized message payload.
    #[redact(level = "secret")]
    pub message_value: String,

    /// UTC submission timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Default for MqFailedTask {
    fn default() -> Self {
        Self {
            id: None,
            topic: String::new(),
            tag: String::new(),
            r#type: MqType::Produce,
            message_id: String::new(),
            message_key: String::new(),
            message_value: String::new(),
            create_time: None,
            delete_time: None,
        }
    }
}
