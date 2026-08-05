// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Model, PartialEq, Eq, Serialize)]
/// Represents the DictEntryInfo domain type.
pub struct DictEntryInfo {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    /// The code value associated with this model.
    pub code: String,
    /// The name value associated with this model.
    pub name: String,
    #[model(identifier)]
    /// The dict_id value associated with this model.
    pub dict_id: Option<i64>,
    /// The params value associated with this model.
    pub params: Vec<String>,
    #[model(time(precision = second, normalization = utc))]
    /// The delete_time value associated with this model.
    pub delete_time: Option<DateTime<Utc>>,
}
