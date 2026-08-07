// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::Payload;
use crate::commons::State;
use crate::contact::Address;
use crate::contact::Location;
use crate::device::DeviceType;
use crate::device::Hardware;
use crate::device::Software;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;
use crate::person::UserInfo;
/// Represents the Device domain type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// The id value associated with this model.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// The code value associated with this model.
    #[model(text(min_chars=1,max_chars=64,repertoire=ascii))]
    pub code: String,

    /// The name value associated with this model.
    #[model(text(min_chars = 1, max_chars = 128))]
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

    /// The binding_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
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

    /// The register_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub register_time: Option<DateTime<Utc>>,

    /// The last_startup_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub last_startup_time: Option<DateTime<Utc>>,

    /// The last_heartbeat_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub last_heartbeat_time: Option<DateTime<Utc>>,

    /// The create_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// The modify_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// The delete_time value associated with this model.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
