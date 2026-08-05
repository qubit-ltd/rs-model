// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment participant snapshots.

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commons::CredentialInfo,
    contact::Phone,
    payment::{
        Account,
        ParticipantType,
    },
};

/// Identifying, contact, account, and category information for a payer or
/// payee.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Participant {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Participant classification.
    pub r#type: Option<ParticipantType>,
    /// Display name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional identity credential.
    pub credential: Option<CredentialInfo>,
    /// Optional mobile telephone number.
    pub mobile: Option<Phone>,
    /// Optional landline telephone number.
    pub phone: Option<Phone>,
    /// Optional email address.
    #[redact(level = "secret")]
    pub email: Option<String>,
    /// Optional payment account.
    pub account: Option<Account>,
    /// Optional participant category.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,
}
