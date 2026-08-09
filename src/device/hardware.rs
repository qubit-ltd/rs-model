// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Hardware identity and network-interface inventory for devices.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::device::SimCard;
/// Hardware attributes used to identify and describe a physical device.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Hardware {
    /// Persisted hardware-record identifier; the default value denotes no record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Marketing or device-reported hardware name, if available.
    pub name: Option<String>,

    /// Manufacturer model designation, if reported.
    pub model: Option<String>,

    /// Device brand, if reported.
    pub brand: Option<String>,

    /// Hardware manufacturer, if reported.
    pub manufacturer: Option<String>,

    /// Product designation, if reported.
    pub product: Option<String>,

    /// Firmware version or build, if reported.
    pub firmware: Option<String>,

    /// Mainboard identifier, if reported.
    pub board: Option<String>,

    /// Low-level hardware platform name, if reported.
    pub hardware: Option<String>,

    /// CPU application binary interfaces supported by the device.
    pub supported_abis: Vec<String>,

    /// Ethernet MAC addresses reported by the device; empty when none are known.
    pub ethernet_mac_addresses: Vec<String>,

    /// Wi-Fi MAC addresses reported by the device; empty when none are known.
    pub wifi_mac_addresses: Vec<String>,

    /// SIM cards or slots discovered on the device; empty when none are reported.
    pub sim_cards: Vec<SimCard>,

    /// Manufacturer serial number, if reported.
    #[redact(level = "secret")]
    pub serial: Option<String>,

    /// Derived unique device identifier, if one has been calculated.
    #[redact(level = "secret")]
    pub udid: Option<String>,
}
