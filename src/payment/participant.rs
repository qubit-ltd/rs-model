// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment participant snapshots.

use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::payment::Account;
use crate::payment::ParticipantType;

/// Identifying, contact, account, and category information for a payer or
/// payee.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Participant {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

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
