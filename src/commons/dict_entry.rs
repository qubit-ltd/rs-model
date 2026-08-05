// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use super::DictEntryInfo;
use crate::mixin::StatefulInfo;
use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
/// Represents the DictEntry domain type.
pub struct DictEntry {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    /// The dict value associated with this model.
    pub dict: StatefulInfo,
    /// The code value associated with this model.
    pub code: String,
    /// The name value associated with this model.
    pub name: String,
    /// The description value associated with this model.
    pub description: Option<String>,
    /// The comment value associated with this model.
    pub comment: Option<String>,
    #[model(opaque)]
    /// The parent value associated with this model.
    pub parent: Option<Box<DictEntryInfo>>,
    #[model(time(precision=second,normalization=utc))]
    /// The create_time value associated with this model.
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    /// The modify_time value associated with this model.
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The delete_time value associated with this model.
    pub delete_time: Option<DateTime<Utc>>,
}
