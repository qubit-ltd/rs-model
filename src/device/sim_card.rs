// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use crate::contact::{
    Location,
    Phone,
};
use crate::device::{
    DataNetworkType,
    SimCardStatus,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
/// Represents the SimCard domain type.
pub struct SimCard {
    #[model(identifier)]
    /// The id value associated with this model.
    pub id: Option<i64>,
    #[redact(level = "secret")]
    /// The iccid value associated with this model.
    pub iccid: String,
    #[redact(level = "secret")]
    /// The imei value associated with this model.
    pub imei: Option<String>,
    #[redact(level = "secret")]
    /// The meid value associated with this model.
    pub meid: Option<String>,
    /// The phone value associated with this model.
    pub phone: Option<Phone>,
    /// The operator value associated with this model.
    pub operator: Option<String>,
    /// The country value associated with this model.
    pub country: Option<String>,
    /// The location value associated with this model.
    pub location: Option<Location>,
    /// The network_type value associated with this model.
    pub network_type: Option<DataNetworkType>,
    /// The status value associated with this model.
    pub status: Option<SimCardStatus>,
}
