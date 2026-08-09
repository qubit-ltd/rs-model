// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Availability and appointment states for an individual service entitlement.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The current availability or appointment progress of a user's service item.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserServiceState {
    /// The entitlement is valid and may be scheduled or used.
    Valid,
    /// The entitlement is no longer valid for use.
    Invalid,
    /// An appointment is being arranged or processed.
    AppointmentProgress,
    /// The appointment completed successfully.
    AppointmentSuccess,
}
