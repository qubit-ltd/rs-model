// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Complete person records.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Blood;
use super::Education;
use super::Ethnic;
use super::Gender;
use super::Incoming;
use super::Industry;
use super::JobTitle;
use super::Marriage;
use super::PersonIdentity;
use super::Politics;
use super::Religion;
use super::SexOrientation;
use crate::commons::Category;
use crate::commons::Credential;
use crate::commons::CredentialInfo;
use crate::commons::Source;
use crate::contact::City;
use crate::contact::Contact;
use crate::contact::Country;
use crate::contact::Province;
use crate::medical::MedicareType;
use crate::mixin::StatefulInfo;
use crate::order::Buyer;
use crate::order::Client;
use crate::order::Consignee;
use crate::person::PersonInfo;
use crate::upload::Attachment;

/// A person's complete demographic, contact, and administrative record.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(
    unique(name = "person_username", fields(username)),
    unique(name = "person_credential", fields(credential))
)]
pub struct Person {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Optional data-source information.
    #[model(reference(target = Source, target_field = info), index, opaque)]
    pub source: Option<InfoWithEntity>,

    /// Optional classification information.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub category: Option<InfoWithEntity>,

    /// Real name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional globally unique registered user name.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: Option<String>,

    /// Optional gender.
    #[model(index)]
    pub gender: Option<Gender>,

    /// Optional birth date.
    #[model(index)]
    pub birthday: Option<NaiveDate>,

    /// Optional birth time.
    #[model(index)]
    pub birth_time: Option<NaiveTime>,

    /// Optional birth-country information.
    #[model(reference(target = Country, target_field = info), index, opaque)]
    pub birth_country: Option<Info>,

    /// Optional birth-province information.
    #[model(reference(target = Province, target_field = info), index, opaque)]
    pub birth_province: Option<Info>,

    /// Optional birth-city information.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub birth_city: Option<Info>,

    /// Optional identity credential.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub credential: Option<CredentialInfo>,

    /// Whether the person has medical insurance.
    #[model(index)]
    pub has_medicare: Option<bool>,

    /// Optional medical-insurance classification.
    #[model(index)]
    pub medicare_type: Option<MedicareType>,

    /// Optional medical-insurance card.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub medicare_card: Option<CredentialInfo>,

    /// Optional medical-insurance city.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub medicare_city: Option<Info>,

    /// Whether the person has social security.
    #[model(index)]
    pub has_social_security: Option<bool>,

    /// Optional social-security card.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub social_security_card: Option<CredentialInfo>,

    /// Optional social-security city.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub social_security_city: Option<Info>,

    /// Optional contact details.
    #[model(index)]
    #[redact(nested)]
    pub contact: Option<Contact>,

    /// Optional guardian information.
    #[model(reference(target = Person, target_field = info))]
    #[redact(nested)]
    pub guardian: Option<PersonInfo>,

    /// Optional education level.
    pub education: Option<Education>,

    /// Optional ethnicity.
    pub ethnic: Option<Ethnic>,

    /// Optional blood type.
    pub blood: Option<Blood>,

    /// Optional marital status.
    pub marriage: Option<Marriage>,

    /// Whether the person has children.
    #[model(index)]
    pub has_child: Option<bool>,

    /// Optional sexual orientation.
    pub sex_orientation: Option<SexOrientation>,

    /// Optional religion.
    pub religion: Option<Religion>,

    /// Optional political affiliation.
    pub politics: Option<Politics>,

    /// Optional industry.
    pub industry: Option<Industry>,

    /// Optional occupation.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub job: Option<String>,

    /// Optional professional title.
    pub job_title: Option<JobTitle>,

    /// Optional income band.
    pub incoming: Option<Incoming>,

    /// Optional employer information.
    pub organization: Option<StatefulInfo>,

    /// Optional height in centimetres.
    pub height: Option<i32>,

    /// Optional weight in kilograms.
    pub weight: Option<i32>,

    /// Optional allergy history.
    pub allergic_history: Option<String>,

    /// Optional portrait photograph.
    #[model(reference(target = Attachment, target_field = id, must_exist = false))]
    pub photo: Option<Attachment>,

    /// Optional comment.
    pub comment: Option<String>,

    /// Whether this is test data.
    #[model(index)]
    pub test: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Person {
    /// Assigns the fields represented by an order client projection.
    pub fn assign_client(&mut self, client: &Client) {
        self.id = client.id;
        self.name = client.name.clone();
        self.gender = client.gender;
        self.birthday = client.birthday;
        self.credential = client.credential.clone();
        self.has_medicare = client.has_medicare;
        self.medicare_card = client.medicare_card.clone();
        self.medicare_city = client.medicare_city.clone();
        self.has_social_security = client.has_social_security;
        self.medicare_type = client.medicare_type;
        self.social_security_card = client.social_security_card.clone();
        self.social_security_city = client.social_security_city.clone();
        self.contact = Contact::create(
            None,
            client.mobile.clone(),
            client.email.clone(),
            None,
            None,
        );
        self.guardian = client.guardian.clone();
    }

    /// Assigns the fields represented by compact person information.
    pub fn assign_info(&mut self, info: &PersonInfo) {
        self.id = info.id;
        self.name = info.name.clone();
        self.gender = info.gender;
        self.birthday = info.birthday;
        self.credential = info.credential.clone();
        self.contact = Contact::create(None, info.mobile.clone(), info.email.clone(), None, None);
        self.test = info.test;
        self.delete_time = info.delete_time;
    }

    /// Assigns the fields represented by a saved consignee.
    pub fn assign_consignee(&mut self, consignee: &Consignee) {
        self.id = consignee.id;
        self.name = consignee.name.clone();
        self.credential = consignee.credential.clone();
        self.contact = Some(Contact {
            mobile: Some(consignee.mobile.clone()),
            email: consignee.email.clone(),
            address: Some(consignee.address.clone()),
            ..Contact::default()
        });
    }

    /// Assigns the fields represented by an order buyer.
    pub fn assign_buyer(&mut self, buyer: &Buyer) {
        self.id = buyer.id;
        self.name = buyer.name.clone();
        self.credential = buyer.credential.clone();
        self.gender = buyer.gender;
        self.birthday = buyer.birthday;
        self.contact = Contact::create(None, buyer.mobile.clone(), buyer.email.clone(), None, None);
    }

    /// Returns this person's compact information projection.
    #[must_use]
    pub fn info(&self) -> PersonInfo {
        PersonInfo {
            id: self.id,
            name: self.name.clone(),
            username: self.username.clone(),
            gender: self.gender,
            birthday: self.birthday,
            credential: self.credential.clone(),
            mobile: self
                .contact
                .as_ref()
                .and_then(|contact| contact.mobile.clone()),
            email: self
                .contact
                .as_ref()
                .and_then(|contact| contact.email.clone()),
            photo: self.photo.clone(),
            test: self.test,
            delete_time: self.delete_time,
        }
    }

    /// Applies compact person information to this record.
    pub fn set_info(&mut self, info: &PersonInfo) {
        self.assign_info(info);
    }

    /// Reports whether either benefit-coverage flag is explicitly true.
    #[must_use]
    pub fn has_medicare_or_social_security(&self) -> bool {
        self.has_medicare.unwrap_or(false) || self.has_social_security.unwrap_or(false)
    }

    /// Reports whether another projection identifies the same person.
    ///
    /// Persisted identifiers take precedence whenever both sides have one;
    /// credentials are considered only when at least one identifier is absent.
    #[must_use]
    pub fn is_same<T: PersonIdentity + ?Sized>(&self, other: &T) -> bool {
        match (self.id, other.person_id()) {
            (id, Some(other_id)) => id.value() as i64 == other_id,
            _ => self
                .credential
                .as_ref()
                .zip(other.person_credential())
                .is_some_and(|(credential, other)| credential.is_same(other)),
        }
    }
}

macro_rules! impl_person_identity {
    ($($type:ty),+ $(,)?) => {
        $(
            impl PersonIdentity for $type {
                fn person_id(&self) -> Option<i64> {
                    Some(self.id.value() as i64)
                }

                fn person_credential(&self) -> Option<&CredentialInfo> {
                    self.credential.as_ref()
                }
            }
        )+
    };
}

impl_person_identity!(Person, PersonInfo, Client, Consignee, Buyer);
