// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Device classification types.

#[allow(unused_imports)]
use super::{
    DataNetworkType,
    SimCardStatus,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Device categories and radio-network classifications are kept together
/// because they form the compact vocabulary used by device telemetry.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    /// The IotBox classification.
    IotBox,
    /// The RadarBedMonitor classification.
    RadarBedMonitor,
}
