// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Buyer snapshots.

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::Gender;

/// Identifying and contact information for the person placing an order.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Buyer {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Optional persisted user identifier.
    pub user_id: Option<i64>,

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
