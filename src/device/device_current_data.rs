// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

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
/// Represents the DeviceCurrentData domain type.
pub struct DeviceCurrentData {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    /// The device_code value associated with this model.
    pub device_code: String,
    /// The msg_id value associated with this model.
    pub msg_id: Option<i64>,
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
