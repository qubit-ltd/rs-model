// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use crate::{
    commons::{
        Payload,
        State,
    },
    contact::{
        Address,
        Location,
    },
    device::{
        DeviceType,
        Hardware,
        Software,
    },
    mixin::StatefulInfo,
    person::{
        PersonInfo,
        UserInfo,
    },
};
use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
/// Represents the Device domain type.
pub struct Device {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    #[model(text(min_chars=1,max_chars=64,repertoire=ascii))]
    /// The code value associated with this model.
    pub code: String,
    #[model(text(min_chars = 1, max_chars = 128))]
    /// The name value associated with this model.
    pub name: String,
    /// The app value associated with this model.
    pub app: StatefulInfo,
    /// The description value associated with this model.
    pub description: Option<String>,
    /// The device_type value associated with this model.
    pub device_type: DeviceType,
    /// The hardware value associated with this model.
    pub hardware: Option<Hardware>,
    /// The operating_system value associated with this model.
    pub operating_system: Option<Software>,
    /// The softwares value associated with this model.
    pub softwares: Vec<Software>,
    /// The location value associated with this model.
    pub location: Option<Location>,
    /// The deploy_address value associated with this model.
    pub deploy_address: Option<Address>,
    /// The ip_address value associated with this model.
    pub ip_address: Option<String>,
    /// The owner value associated with this model.
    pub owner: Option<PersonInfo>,
    #[model(time(precision=second,normalization=utc))]
    /// The binding_time value associated with this model.
    pub binding_time: Option<DateTime<Utc>>,
    /// The binder value associated with this model.
    pub binder: Option<UserInfo>,
    /// The state value associated with this model.
    pub state: State,
    /// The payloads value associated with this model.
    pub payloads: Vec<Payload>,
    /// The comment value associated with this model.
    pub comment: Option<String>,
    /// The test value associated with this model.
    pub test: bool,
    #[model(time(precision=second,normalization=utc))]
    /// The register_time value associated with this model.
    pub register_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The last_startup_time value associated with this model.
    pub last_startup_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The last_heartbeat_time value associated with this model.
    pub last_heartbeat_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The create_time value associated with this model.
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    /// The modify_time value associated with this model.
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    /// The delete_time value associated with this model.
    pub delete_time: Option<DateTime<Utc>>,
}
