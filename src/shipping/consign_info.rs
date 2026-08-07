// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Consignor and consignee values.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{
    commons::CredentialInfo,
    contact::{Address, Phone},
};

/// Identifying and contact information for one side of a shipment.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
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
