// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Device classification types.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;


/// SIM-card availability and lock state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SimCardStatus {
    /// The Unknown classification.
    Unknown,
    /// The Absent classification.
    Absent,
    /// The PinRequired classification.
    PinRequired,
    /// The PukRequired classification.
    PukRequired,
    /// The NetworkLocked classification.
    NetworkLocked,
    /// The Ready classification.
    Ready,
    /// The NotReady classification.
    NotReady,
    /// The PermDisabled classification.
    PermDisabled,
    /// The CardIoError classification.
    CardIoError,
    /// The CardRestricted classification.
    CardRestricted,
}
