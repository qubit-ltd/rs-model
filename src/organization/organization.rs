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
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::TaxPayerType;
use crate::commons::Category;
use crate::commons::Credential;
use crate::commons::CredentialInfo;
use crate::commons::Payload;
use crate::commons::State;
use crate::contact::Contact;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;
use crate::product::Seller;

/// A company, medical institution, school, government body, or other
/// organization.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Organization {
    /// Database identifier for this organization; default denotes an unsaved record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Globally unique display name.
    #[model(index, unique(ignore_case), text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Business category used to classify the organization in directory and workflow views.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub category: Option<InfoWithEntity>,

    /// Parent organization in the organizational hierarchy, when this is a subunit.
    #[model(reference(target = Organization, target_field = info), index, opaque)]
    pub parent: Option<StatefulInfo>,

    /// Lifecycle state.
    #[model(index)]
    pub state: State,

    /// Icon URI used to visually identify the organization in clients.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Public-facing description of the organization's purpose or services.
    pub description: Option<String>,

    /// Internal administrator note not represented by structured organization fields.
    pub comment: Option<String>,

    /// Official contact channels for reaching the organization.
    #[model(index, opaque)]
    #[redact(nested)]
    pub contact: Option<Contact>,

    /// Primary registration or identity credential for the legal organization.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    pub credential: Option<CredentialInfo>,

    /// Licenses or qualification credentials that authorize the organization's activities.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    pub licenses: Option<Vec<CredentialInfo>>,

    /// Person legally responsible for or representing the organization.
    #[model(index, opaque)]
    #[redact(nested)]
    pub principal: Option<PersonInfo>,

    /// Taxpayer category used by invoicing and fiscal processes.
    pub tax_payer_type: Option<TaxPayerType>,

    /// Tax registration number used for fiscal documents and validation.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub tax_number: Option<String>,

    /// Application-defined key-value extensions retained with the organization record.
    #[model(
        reference(target = Payload, target_field = id, must_exist = false),
        sequence(max_items = 10)
    )]
    pub payloads: Option<Vec<Payload>>,

    /// Marks a platform-provided organization that administrators should not treat as ordinary data.
    #[model(index)]
    pub predefined: bool,

    /// Marks non-production organization data excluded from live operational reporting.
    #[model(index)]
    pub test: bool,

    /// UTC instant when the organization record was created.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// UTC instant of the most recent persisted organization update.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC instant of soft deletion; absent while the organization remains active.
    #[model(index, time(precision = second, normalization = utc))]
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
