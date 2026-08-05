// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use crate::{
    commons::{Payload, State},
    contact::{Address, Location},
    device::{DeviceType, Hardware, Software},
    mixin::StatefulInfo,
    person::{PersonInfo, UserInfo},
};
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Device {
    #[model(identifier)]
    pub id: Option<i64>,
    #[model(text(min_chars=1,max_chars=64,repertoire=ascii))]
    pub code: String,
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    pub app: StatefulInfo,
    pub description: Option<String>,
    pub device_type: DeviceType,
    pub hardware: Option<Hardware>,
    pub operating_system: Option<Software>,
    pub softwares: Vec<Software>,
    pub location: Option<Location>,
    pub deploy_address: Option<Address>,
    pub ip_address: Option<String>,
    pub owner: Option<PersonInfo>,
    #[model(time(precision=second,normalization=utc))]
    pub binding_time: Option<DateTime<Utc>>,
    pub binder: Option<UserInfo>,
    pub state: State,
    pub payloads: Vec<Payload>,
    pub comment: Option<String>,
    pub test: bool,
    #[model(time(precision=second,normalization=utc))]
    pub register_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub last_startup_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub last_heartbeat_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
