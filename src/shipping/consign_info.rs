// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Consignor and consignee values.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Address;
use crate::contact::Phone;

/// Identifying and contact information for one side of a shipment.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ConsignInfo {
    /// Person or organization name.
    pub name: String,

    /// Mobile number.
    pub mobile: Phone,

    /// Optional email address.
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Optional credential used for identity-verified delivery.
    pub credential: Option<CredentialInfo>,

    /// Shipping address.
    pub address: Address,
}
