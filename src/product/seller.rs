// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Seller information values.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commons::CredentialInfo,
    contact::{
        Address,
        Phone,
    },
};

/// Contact and identifying information for a product seller.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Seller {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique seller code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Seller name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional landline telephone number.
    pub phone: Option<Phone>,
    /// Optional mobile telephone number.
    pub mobile: Option<Phone>,
    /// Optional email address.
    #[model(sensitive(redact), text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    pub email: Option<String>,
    /// Optional website URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional personal or organization credential summary.
    pub credential: Option<CredentialInfo>,
    /// Optional contact address.
    pub address: Option<Address>,
}
