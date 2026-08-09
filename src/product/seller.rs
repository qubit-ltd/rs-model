// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Seller information values.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Address;
use crate::contact::Phone;

/// Contact and identifying information for a product seller.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Seller {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique seller code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64))]
    pub code: String,

    /// Seller name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional landline telephone number.
    pub phone: Option<Phone>,

    /// Optional mobile telephone number.
    pub mobile: Option<Phone>,

    /// Optional email address.
    #[model(sensitive(redact), text(min_chars = 1, max_chars = 512))]
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
