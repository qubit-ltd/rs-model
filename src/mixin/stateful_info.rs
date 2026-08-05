//! Stateful basic-information snapshots.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::State;

/// Basic identifying information together with a lifecycle state.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct StatefulInfo {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Stable code for the referenced entity.
    pub code: String,
    /// Human-readable name for the referenced entity.
    pub name: String,
    /// Optional lifecycle state.
    pub state: Option<State>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl StatefulInfo {
    /// Creates a stateful information snapshot.
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
            id,
            code,
            name,
            state,
            delete_time,
        }
    }
}
