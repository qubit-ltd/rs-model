// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Common settlement record fields.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::mixin::StatefulInfo;

/// Shared persisted fields carried by concrete settlement records.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Settlement {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Application that owns this settlement.
    pub app: StatefulInfo,

    /// Organization that owns this settlement.
    pub organization: StatefulInfo,

    /// Optional settlement remark.
    pub remark: Option<String>,

    /// UTC creation timestamp.
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    pub delete_time: Option<DateTime<Utc>>,
}
