// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Organization aggregate roots.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Emptyful;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Category;
use crate::commons::Credential;
use crate::commons::CredentialInfo;
use crate::commons::Payload;
use crate::commons::State;
use crate::contact::Contact;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;
use crate::product::Seller;
use super::TaxPayerType;

/// A company, medical institution, school, government body, or other
/// organization.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(
    unique(name = "organization_code", fields(code), ignore_case(code)),
    unique(name = "organization_name", fields(name), ignore_case(name))
)]
pub struct Organization {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Globally unique ASCII code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Globally unique display name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<InfoWithEntity>,

    /// Optional parent-organization information.
    #[model(reference(target = Organization, target_field = info), index, opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<StatefulInfo>,

    /// Lifecycle state.
    #[model(index)]
    pub state: State,

    /// Optional ASCII icon path or URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Optional user-facing description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional administrator comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Optional contact details.
    #[model(index, opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// Optional primary identity credential.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialInfo>,

    /// Optional qualification credentials.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<CredentialInfo>>,

    /// Optional legal representative or responsible person.
    #[model(index, opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<PersonInfo>,

    /// Optional tax-payer classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_payer_type: Option<TaxPayerType>,

    /// Optional ASCII tax number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,

    /// Optional extension payloads.
    #[model(
        reference(target = Payload, target_field = id, must_exist = false),
        sequence(max_items = 10)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<Payload>>,

    /// Whether this is predefined reference data.
    #[model(index)]
    pub predefined: bool,

    /// Whether this is development or test data.
    #[model(index)]
    pub test: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Organization {
    /// Projects this organization into its stateful information form.
    #[must_use]
    pub fn info(&self) -> StatefulInfo {
        StatefulInfo {
            id: self.id,
            code: self.code.clone(),
            name: self.name.clone(),
            state: Some(self.state),
            delete_time: self.delete_time,
        }
    }

    /// Assigns the fields represented by a product seller.
    pub fn assign_seller(&mut self, seller: &Seller) {
        self.id = seller.id;
        self.code.clone_from(&seller.code);
        self.name.clone_from(&seller.name);
        self.credential.clone_from(&seller.credential);
        self.contact = Contact::create(
            seller.phone.clone(),
            seller.mobile.clone(),
            seller.email.clone(),
            seller.url.clone(),
            seller.address.clone(),
        );
    }

    /// Returns whether every property has its source empty representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

impl Emptyful for Organization {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Organization {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.icon.normalize();
        self.description.normalize();
        self.comment.normalize();
        self.contact.normalize();
        self.tax_number.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
