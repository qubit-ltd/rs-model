// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Compact identity snapshots that include lifecycle state.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::State;

/// Compact identity projection enriched with lifecycle and soft-deletion state.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StatefulInfo {
    /// Persisted identifier, using the neutral ID value when no identifier is supplied.
    #[model(opaque)]
    pub id: Id,

    /// Stable code of the referenced entity.
    pub code: String,

    /// Human-readable display name of the referenced entity.
    pub name: String,

    /// Lifecycle state, or `None` when this projection carries no state.
    pub state: Option<State>,

    /// UTC soft-deletion time, or `None` while the entity is not deleted.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl StatefulInfo {
    /// Creates a stateful identity snapshot without altering the supplied code or name.
    ///
    /// # Parameters
    ///
    /// * `id` - Optional persisted identifier.
    /// * `code` - Stable entity code.
    /// * `name` - Human-readable entity name.
    /// * `state` - Optional lifecycle state.
    /// * `delete_time` - Optional UTC soft-deletion timestamp.
    ///
    /// # Returns
    ///
    /// A snapshot containing the supplied fields unchanged.
    #[must_use]
    pub fn new(
        id: Option<i64>,
        code: String,
        name: String,
        state: Option<State>,
        delete_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: id.map_or_else(Id::default, |value| Id::from(value as u64)),
            code,
            name,
            state,
            delete_time,
        }
    }
}
