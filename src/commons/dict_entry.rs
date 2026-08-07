// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use super::DictEntryInfo;
use crate::mixin::StatefulInfo;
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
/// Represents the DictEntry domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DictEntry {
    /// The id value associated with this model.
    #[model(identifier)]
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

    /// The parent value associated with this model.
    #[model(opaque)]
    pub parent: Option<Box<DictEntryInfo>>,

    /// The create_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// The modify_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// The delete_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
