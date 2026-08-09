// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Subscriber-identity module inventory associated with device hardware.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Location;
use crate::contact::Phone;
use crate::device::DataNetworkType;
use crate::device::SimCardStatus;
/// A SIM card or SIM slot and its carrier, identity, and readiness details.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct SimCard {
    /// Persisted SIM-record identifier; the default value denotes no stored card.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Integrated Circuit Card Identifier (ICCID) assigned to this card.
    #[redact(level = "secret")]
    pub iccid: String,

    /// IMEI of the associated device slot, if reported; it identifies hardware, not the card.
    #[redact(level = "secret")]
    pub imei: Option<String>,

    /// MEID of the associated device slot, if reported.
    #[redact(level = "secret")]
    pub meid: Option<String>,

    /// Telephone number provisioned on the card, if known.
    pub phone: Option<Phone>,

    /// Mobile network operator name, if reported.
    pub operator: Option<String>,

    /// Country code of the issuing operator, if known.
    pub country: Option<String>,

    /// Current card location, if the network reported one.
    pub location: Option<Location>,

    /// Current mobile data radio technology, if connected.
    pub network_type: Option<DataNetworkType>,

    /// Card readiness or lock state, if reported.
    pub status: Option<SimCardStatus>,
}
