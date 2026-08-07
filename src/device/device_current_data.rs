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
/// Represents the DeviceCurrentData domain type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeviceCurrentData {
    /// The id value associated with this model.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// The device_code value associated with this model.
    pub device_code: String,

    /// The msg_id value associated with this model.
    #[model(opaque)]
    pub msg_id: Id,

    /// The ack value associated with this model.
    pub ack: Option<i32>,

    /// The keep_push value associated with this model.
    pub keep_push: Option<i32>,

    /// The gettime value associated with this model.
    pub gettime: Option<i64>,

    /// The heart_rate value associated with this model.
    pub heart_rate: Vec<i32>,

    /// The respiratory_rate value associated with this model.
    pub respiratory_rate: Vec<i32>,

    /// The body_movement value associated with this model.
    pub body_movement: Vec<i32>,

    /// The move_state value associated with this model.
    pub move_state: Vec<i32>,

    /// The body_status value associated with this model.
    pub body_status: Vec<i32>,

    /// The body_position value associated with this model.
    pub body_position: Vec<i32>,

    /// The onbed_status value associated with this model.
    pub onbed_status: Option<i32>,

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
