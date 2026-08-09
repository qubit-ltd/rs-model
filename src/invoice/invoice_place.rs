// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-issuing locations.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::State;
use crate::mixin::StatefulInfo;

/// A coded invoice-issuing location within an organization.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InvoicePlace {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application that owns this issuing location.
    pub app: StatefulInfo,

    /// Organization that owns this issuing location.
    pub organization: StatefulInfo,

    /// Issuing-location code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Issuing-location name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Current lifecycle state.
    pub state: State,

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
