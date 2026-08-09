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
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::mixin::StatefulInfo;

/// Shared persisted fields carried by concrete settlement records.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settlement {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application that owns this settlement.
    pub app: StatefulInfo,

    /// Organization that owns this settlement.
    pub organization: StatefulInfo,

    /// Optional settlement remark.
    pub remark: Option<String>,

    /// UTC instant at which this record was created.
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    pub delete_time: Option<DateTime<Utc>>,
}
