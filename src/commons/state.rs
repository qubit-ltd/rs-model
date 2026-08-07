// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lifecycle state values shared by domain models.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes the lifecycle state of an entity.
#[derive(Model, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
