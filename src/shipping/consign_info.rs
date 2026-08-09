// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Contact and identity information for either party to a shipment.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Address;
use crate::contact::Phone;

/// Delivery identity, contact channel, and address for a consignor or consignee.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ConsignInfo {
    /// Name of the sending or receiving person or organization.
    pub name: String,

    /// Mobile number used for delivery contact.
    pub mobile: Phone,

    /// Email delivery contact, or `None` when no email address was supplied.
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Credential used for identity-verified delivery, or `None` when verification is unnecessary.
    pub credential: Option<CredentialInfo>,

    /// Physical address at which the shipment is collected or delivered.
    pub address: Address,
}
