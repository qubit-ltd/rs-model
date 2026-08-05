// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DeviceCurrentData {
    #[model(identifier)]
    pub id: Option<i64>,
    pub device_code: String,
    pub msg_id: Option<i64>,
    pub ack: Option<i32>,
    pub keep_push: Option<i32>,
    pub gettime: Option<i64>,
    pub heart_rate: Vec<i32>,
    pub respiratory_rate: Vec<i32>,
    pub body_movement: Vec<i32>,
    pub move_state: Vec<i32>,
    pub body_status: Vec<i32>,
    pub body_position: Vec<i32>,
    pub onbed_status: Option<i32>,
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
