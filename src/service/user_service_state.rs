// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! User medical-service states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes availability and appointment progress for a user's service.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserServiceState {
    /// The service is valid.
    Valid,
    /// The service is invalid.
    Invalid,
    /// An appointment is in progress.
    AppointmentProgress,
    /// An appointment succeeded.
    AppointmentSuccess,
}
