//! Lifecycle state values shared by domain models.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Describes the lifecycle state of an entity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    /// The entity has not been activated.
    Inactive,
    /// The entity is available for ordinary use.
    #[default]
    Normal,
    /// The entity is temporarily unavailable.
    Locked,
    /// The entity is permanently blocked.
    Blocked,
    /// The entity is obsolete.
    Obsoleted,
    /// The entity has been disabled.
    Disabled,
}
