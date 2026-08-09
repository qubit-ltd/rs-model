// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lightweight cross-domain references to registered devices.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::State;
use crate::contact::Location;
use crate::device::DeviceType;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;
/// Compact device projection for references that do not need full inventory data.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeviceInfo {
    /// Persisted device identifier; the default value denotes no stored device.
    pub id: Id,

    /// Stable device code within the managing application.
    #[model(unique)]
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub code: String,

    /// Human-readable device name.
    pub name: String,

    /// Application that manages the device.
    pub app: StatefulInfo,

    /// Current device owner, if the device is bound.
    pub owner: Option<PersonInfo>,

    /// Lifecycle state of the referenced device.
    pub state: State,

    /// Physical category of the device.
    pub device_type: DeviceType,

    /// Last known device location, if reported.
    pub location: Option<Location>,

    /// Whether the device is test data.
    pub test: bool,

    /// UTC time at which the current owner binding was made, if any.
    pub binding_time: Option<DateTime<Utc>>,

    /// UTC device-registration time, if known.
    #[model(index, time(precision = second, normalization = utc))]
    pub register_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, if the device has been deleted.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
