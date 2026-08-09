// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Buyer snapshots.

use chrono::NaiveDate;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::Gender;

/// Identifying and contact information for the person placing an order.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Buyer {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Identifier of the user; its default value means that no related record is stored.
    #[model(opaque)]
    pub user_id: Id,

    /// Buyer name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional identity credential.
    pub credential: Option<CredentialInfo>,

    /// Optional gender.
    pub gender: Option<Gender>,

    /// Optional birthday.
    pub birthday: Option<NaiveDate>,

    /// Optional mobile number.
    pub mobile: Option<Phone>,

    /// Optional email address.
    #[redact(level = "secret")]
    pub email: Option<String>,
}
