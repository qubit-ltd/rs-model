// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! SIM-card readiness, lock, and failure states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// SIM-card availability and lock state.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SimCardStatus {
    /// The state has not been reported or cannot be determined.
    Unknown,
    /// No SIM card is inserted.
    Absent,
    /// The card is locked until its PIN is supplied.
    PinRequired,
    /// The card is locked until its PUK is supplied.
    PukRequired,
    /// A network lock prevents the card from being used.
    NetworkLocked,
    /// The card is present and ready for service.
    Ready,
    /// The card is present but not yet ready for service.
    NotReady,
    /// The card has been permanently disabled.
    PermDisabled,
    /// Communication with the card failed due to an I/O error.
    CardIoError,
    /// Carrier restrictions prevent the card from being used.
    CardRestricted,
}
