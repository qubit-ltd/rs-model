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
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub iccid: String,

    /// IMEI of the associated device slot, if reported; it identifies hardware, not the card.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub imei: Option<String>,

    /// MEID of the associated device slot, if reported.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub meid: Option<String>,

    /// Telephone number provisioned on the card, if known.
    #[model(index)]
    pub phone: Option<Phone>,

    /// Mobile network operator name, if reported.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub operator: Option<String>,

    /// Country code of the issuing operator, if known.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub country: Option<String>,

    /// Current card location, if the network reported one.
    pub location: Option<Location>,

    /// Current mobile data radio technology, if connected.
    #[model(index)]
    pub network_type: Option<DataNetworkType>,

    /// Card readiness or lock state, if reported.
    #[model(index)]
    pub status: Option<SimCardStatus>,
}
