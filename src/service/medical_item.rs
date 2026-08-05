// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical service items.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// A coded medical service entitlement or procedure.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct MedicalItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique service code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Service name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional service description.
    pub description: Option<String>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
