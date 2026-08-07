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

/// Device categories and radio-network classifications are kept together
/// because they form the compact vocabulary used by device telemetry.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    /// The IotBox classification.
    IotBox,
    /// The RadarBedMonitor classification.
    RadarBedMonitor,
}
