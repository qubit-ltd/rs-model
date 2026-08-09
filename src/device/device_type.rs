// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Device categories used by inventory and telemetry records.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Physical device category recorded by the platform.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    /// Internet-of-Things gateway or control box.
    IotBox,
    /// Radar-based bed-monitoring device.
    RadarBedMonitor,
}
