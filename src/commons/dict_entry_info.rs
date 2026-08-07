// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Represents the DictEntryInfo domain type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct DictEntryInfo {
    /// The id value associated with this model.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// The code value associated with this model.
    pub code: String,

    /// The name value associated with this model.
    pub name: String,

    /// The dict_id value associated with this model.
    #[model(identifier)]
    #[model(opaque)]
    pub dict_id: Id,

    /// The params value associated with this model.
    pub params: Vec<String>,

    /// The delete_time value associated with this model.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
