// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Registered device records and their lifecycle telemetry.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::App;
use crate::commons::Payload;
use crate::commons::State;
use crate::contact::Address;
use crate::contact::Location;
use crate::contact::Street;
use crate::device::DeviceType;
use crate::device::Hardware;
use crate::device::Software;
use crate::mixin::StatefulInfo;
use crate::person::Person;
use crate::person::PersonInfo;
use crate::person::User;
use crate::person::UserInfo;
/// A registered device, including its ownership, software inventory, and state.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Device {
    /// Persisted device identifier; the default value denotes an unsaved device.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stable ASCII code that identifies the device within the application.
    #[model(unique)]
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub code: String,

    /// Human-readable device name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that registered or manages this device.
    #[model(reference(target = App, target_field = info))]
    pub app: StatefulInfo,

    /// Optional user-facing description; absent when none was supplied.
    pub description: Option<String>,

    /// Physical category of the device.
    #[model(index)]
    pub device_type: DeviceType,

    /// Optional hardware inventory; absent when it has not been collected.
    pub hardware: Option<Hardware>,

    /// Optional operating-system software record.
    pub operating_system: Option<Software>,

    /// Software installed on the device; an empty collection means none was reported.
    pub softwares: Vec<Software>,

    /// Last known geographic location, if the device reported one.
    pub location: Option<Location>,

    /// Physical deployment address, if assigned. Its nested administrative
    /// references identify the terminal street entity.
    #[model(reference(target = Street, target_field = info), index, opaque)]
    pub deploy_address: Option<Address>,

    /// Current network IP address, if known.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub ip_address: Option<String>,

    /// Person currently responsible for the device, if it is bound.
    #[model(reference(target = Person, target_field = info))]
    pub owner: Option<PersonInfo>,

    /// UTC time at which the current owner binding was established, if any.
    #[model(index, time(precision = second, normalization = utc))]
    pub binding_time: Option<DateTime<Utc>>,

    /// User who established the current owner binding, if recorded.
    #[model(reference(target = User, target_field = info, must_exist = true))]
    pub binder: Option<UserInfo>,

    /// Lifecycle state of this device record.
    #[model(index)]
    pub state: State,

    /// Extension payloads attached to this device; empty when none are stored.
    #[model(
        reference(target = Payload, target_field = id, must_exist = false),
        sequence(min_items = 1, max_items = 10)
    )]
    pub payloads: Vec<Payload>,

    /// Optional administrator note.
    pub comment: Option<String>,

    /// Whether this record is test data rather than a production device.
    pub test: bool,

    /// UTC time at which the device was registered, if known.
    #[model(index, time(precision = second, normalization = utc))]
    pub register_time: Option<DateTime<Utc>>,

    /// Most recent device startup time in UTC, if reported.
    #[model(index, time(precision = second, normalization = utc))]
    pub last_startup_time: Option<DateTime<Utc>>,

    /// Most recent heartbeat time in UTC, if reported.
    #[model(index, time(precision = second, normalization = utc))]
    pub last_heartbeat_time: Option<DateTime<Utc>>,

    /// UTC time at which this record was created.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC time of the most recent update, if the record has been modified.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, if the device has been deleted.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
