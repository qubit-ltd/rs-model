// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Application-to-resource associations.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::App;

/// Associates an application with a typed domain resource.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct AppResource {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Referenced application identifier.
    #[model(reference(target = App, target_field = id))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,

    /// ASCII resource type.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub resource_type: String,

    /// Referenced resource identifier.
    #[model(index)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i64>,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}
