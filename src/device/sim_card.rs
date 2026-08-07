// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Location;
use crate::contact::Phone;
use crate::device::DataNetworkType;
use crate::device::SimCardStatus;
/// Represents the SimCard domain type.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct SimCard {
    /// The id value associated with this model.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

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
