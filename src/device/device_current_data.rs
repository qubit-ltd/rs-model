// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Latest telemetry samples received from a registered device.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
/// Current physiological and occupancy telemetry associated with a device.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeviceCurrentData {
    /// Persisted telemetry-record identifier; the default value denotes no record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Code of the device that emitted this telemetry.
    #[model(index, text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub device_code: String,

    /// Identifier of the source telemetry message.
    #[model(index, opaque)]
    pub msg_id: Id,

    /// Optional acknowledgement value supplied by the device protocol.
    pub ack: Option<i32>,

    /// Optional protocol flag controlling retained push delivery.
    pub keep_push: Option<i32>,

    /// Optional source acquisition time in the device protocol's unit.
    #[model(index)]
    pub gettime: Option<i64>,

    /// Heart-rate samples reported by the device; empty when no sample was reported.
    pub heart_rate: Vec<i32>,

    /// Respiratory-rate samples reported by the device; empty when unavailable.
    pub respiratory_rate: Vec<i32>,

    /// Body-movement samples reported by the device; empty when unavailable.
    pub body_movement: Vec<i32>,

    /// Movement-state samples in the device protocol's encoding.
    pub move_state: Vec<i32>,

    /// Body-status samples in the device protocol's encoding.
    pub body_status: Vec<i32>,

    /// Body-position samples in the device protocol's encoding.
    pub body_position: Vec<i32>,

    /// Optional on-bed state in the device protocol's encoding.
    pub onbed_status: Option<i32>,

    /// UTC time at which this telemetry record was created.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC time of the latest update, if the record was modified.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, if the telemetry record was deleted.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
