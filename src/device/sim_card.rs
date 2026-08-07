// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Location;
use crate::contact::Phone;
use crate::device::DataNetworkType;
use crate::device::SimCardStatus;
/// Represents the SimCard domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct SimCard {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The iccid value associated with this model.
    #[redact(level = "secret")]
    pub iccid: String,

    /// The imei value associated with this model.
    #[redact(level = "secret")]
    pub imei: Option<String>,

    /// The meid value associated with this model.
    #[redact(level = "secret")]
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
