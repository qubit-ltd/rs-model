// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Represents the DictEntryInfo domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct DictEntryInfo {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The code value associated with this model.
    pub code: String,

    /// The name value associated with this model.
    pub name: String,

    /// The dict_id value associated with this model.
    #[model(identifier)]
    pub dict_id: Option<i64>,

    /// The params value associated with this model.
    pub params: Vec<String>,

    /// The delete_time value associated with this model.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
