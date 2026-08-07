// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::State;
use crate::contact::Location;
use crate::device::DeviceType;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;
/// Represents the DeviceInfo domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DeviceInfo {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The code value associated with this model.
    pub code: String,

    /// The name value associated with this model.
    pub name: String,

    /// The app value associated with this model.
    pub app: StatefulInfo,

    /// The owner value associated with this model.
    pub owner: Option<PersonInfo>,

    /// The state value associated with this model.
    pub state: State,

    /// The device_type value associated with this model.
    pub device_type: DeviceType,

    /// The location value associated with this model.
    pub location: Option<Location>,

    /// The test value associated with this model.
    pub test: bool,

    /// The binding_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub binding_time: Option<DateTime<Utc>>,

    /// The register_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub register_time: Option<DateTime<Utc>>,

    /// The delete_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
