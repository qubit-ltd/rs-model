// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use crate::device::SimCard;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
/// Represents the Hardware domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Hardware {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The name value associated with this model.
    pub name: Option<String>,

    /// The model value associated with this model.
    pub model: Option<String>,

    /// The brand value associated with this model.
    pub brand: Option<String>,

    /// The manufacturer value associated with this model.
    pub manufacturer: Option<String>,

    /// The product value associated with this model.
    pub product: Option<String>,

    /// The firmware value associated with this model.
    pub firmware: Option<String>,

    /// The board value associated with this model.
    pub board: Option<String>,

    /// The hardware value associated with this model.
    pub hardware: Option<String>,

    /// The supported_abis value associated with this model.
    pub supported_abis: Vec<String>,

    /// The ethernet_mac_addresses value associated with this model.
    pub ethernet_mac_addresses: Vec<String>,

    /// The wifi_mac_addresses value associated with this model.
    pub wifi_mac_addresses: Vec<String>,

    /// The sim_cards value associated with this model.
    pub sim_cards: Vec<SimCard>,

    /// The serial value associated with this model.
    #[redact(level = "secret")]
    pub serial: Option<String>,

    /// The udid value associated with this model.
    #[redact(level = "secret")]
    pub udid: Option<String>,
}
