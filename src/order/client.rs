// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Product recipient snapshots.

use chrono::NaiveDate;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::commons::Kinship;
use crate::contact::Phone;
use crate::medical::MedicareType;
use crate::order::ReturnStatus;
use crate::person::Gender;
use crate::person::PersonInfo;

/// Identity, benefits, guardian, and return information for a product client.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Client {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Client name.
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

    /// Optional Medicare ownership state.
    pub has_medicare: Option<bool>,

    /// Optional Medicare classification.
    pub medicare_type: Option<MedicareType>,

    /// Optional Medicare credential.
    pub medicare_card: Option<CredentialInfo>,

    /// Optional Medicare city information.
    #[model(opaque)]
    pub medicare_city: Option<Info>,

    /// Optional social-security ownership state.
    pub has_social_security: Option<bool>,

    /// Optional social-security credential.
    pub social_security_card: Option<CredentialInfo>,

    /// Optional social-security city information.
    #[model(opaque)]
    pub social_security_city: Option<Info>,

    /// Optional guardian information.
    pub guardian: Option<PersonInfo>,

    /// Optional return state for this client's order item.
    pub return_status: Option<ReturnStatus>,

    /// Optional relationship to the buyer.
    pub kinship: Option<Kinship>,

    /// Optional ordered payload entries.
    #[model(opaque)]
    pub payload: Option<Vec<(String, String)>>,
}
