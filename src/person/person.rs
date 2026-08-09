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
use super::User;
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
use crate::organization::Organization;
use crate::person::PersonInfo;
use crate::upload::Attachment;

/// A person's complete demographic, contact, and administrative record.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(
    unique(name = "person_username", fields(username), ignore_case(username)),
    unique(name = "person_credential", fields(credential))
)]
pub struct Person {
    /// Database identifier for this person; the default value denotes an unsaved profile.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Originating system or import source used to trace how this profile was created.
    #[model(reference(target = Source, target_field = info), index, opaque)]
    pub source: Option<InfoWithEntity>,

    /// Business category used to segment the person for workflows and reporting.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub category: Option<InfoWithEntity>,

    /// Legal or commonly used real name shown on the person's profile.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Globally unique login name when the person is associated with a user account.
    #[model(
        reference(target = User, target_field = username),
        index,
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    pub username: Option<String>,

    /// Self-reported or source-supplied gender for demographic and service records.
    #[model(index)]
    pub gender: Option<Gender>,

    /// Calendar birth date used for age-sensitive eligibility and demographic processing.
    #[model(index)]
    pub birthday: Option<NaiveDate>,

    /// Reported local time of birth when the source provides it.
    #[model(index)]
    pub birth_time: Option<NaiveTime>,

    /// Country recorded as the person's place of birth.
    #[model(reference(target = Country, target_field = info), index, opaque)]
    pub birth_country: Option<Info>,

    /// Province or equivalent subdivision recorded as the person's place of birth.
    #[model(reference(target = Province, target_field = info), index, opaque)]
    pub birth_province: Option<Info>,

    /// City recorded as the person's place of birth.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub birth_city: Option<Info>,

    /// Identity credential used to recognize the person across administrative processes.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub credential: Option<CredentialInfo>,

    /// Indicates whether the person is covered by a medical-insurance program.
    #[model(index)]
    pub has_medicare: Option<bool>,

    /// Medical-insurance scheme that determines the person's applicable benefits.
    #[model(index)]
    pub medicare_type: Option<MedicareType>,

    /// Credential identifying the person's medical-insurance card, when issued.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub medicare_card: Option<CredentialInfo>,

    /// City administering the person's medical-insurance enrollment.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub medicare_city: Option<Info>,

    /// Indicates whether the person participates in a social-security program.
    #[model(index)]
    pub has_social_security: Option<bool>,

    /// Credential identifying the person's social-security card, when issued.
    #[model(reference(target = Credential, target_field = info, must_exist = false))]
    #[redact(nested)]
    pub social_security_card: Option<CredentialInfo>,

    /// City administering the person's social-security enrollment.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub social_security_city: Option<Info>,

    /// Contact channels used to reach the person for services, care, or notifications.
    #[model(index)]
    #[redact(nested)]
    pub contact: Option<Contact>,

    /// Guardian or responsible person for minors or people needing representation.
    #[model(reference(target = Person, target_field = info))]
    #[redact(nested)]
    pub guardian: Option<PersonInfo>,

    /// Highest reported educational attainment for demographic profiling.
    pub education: Option<Education>,

    /// Ethnic-group value retained for demographic reporting where permitted.
    pub ethnic: Option<Ethnic>,

    /// Reported ABO blood group for clinical and emergency-reference workflows.
    pub blood: Option<Blood>,

    /// Current or most recently reported marital-status classification.
    pub marriage: Option<Marriage>,

    /// Indicates whether the person reports having one or more children.
    #[model(index)]
    pub has_child: Option<bool>,

    /// Self-disclosed sexual orientation, retained only when the source provides it.
    pub sex_orientation: Option<SexOrientation>,

    /// Religious affiliation supplied for demographic or service-preference use.
    pub religion: Option<Religion>,

    /// Political affiliation supplied for demographic or administrative use.
    pub politics: Option<Politics>,

    /// Industry sector of the person's current or primary employment.
    pub industry: Option<Industry>,

    /// Free-text occupation or job function not represented by the industry category.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub job: Option<String>,

    /// Professional-title or management-seniority band associated with the person's work.
    pub job_title: Option<JobTitle>,

    /// Reported annual-income band used for demographic segmentation.
    pub incoming: Option<Incoming>,

    /// Employer organization associated with the person's current role.
    #[model(reference(target = Organization, target_field = info))]
    pub organization: Option<StatefulInfo>,

    /// Height in centimetres, when collected for clinical or service eligibility use.
    pub height: Option<i32>,

    /// Weight in kilograms, when collected for clinical or service eligibility use.
    pub weight: Option<i32>,

    /// Free-text allergy history that may affect care or service delivery.
    pub allergic_history: Option<String>,

    /// Profile photograph used to help staff visually identify the person.
    #[model(reference(target = Attachment, target_field = id, must_exist = false))]
    pub photo: Option<Attachment>,

    /// Staff-maintained note that does not fit a structured profile field.
    pub comment: Option<String>,

    /// Marks synthetic or non-production data that must be excluded from live operations.
    #[model(index)]
    pub test: bool,

    /// UTC instant when the person profile was first created.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent persisted profile update.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC instant when the profile was soft deleted; absence means it remains active.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Person {
    /// Merges an order-client projection into this person record.
    ///
    /// Overwrites `id`, `name`, `gender`, `birthday`, `credential`, all medical- and
    /// social-security fields, and `guardian`. Replaces the entire `contact` value with a new
    /// projection containing only the client's mobile and email channels. All other fields,
    /// including profile, employment, and lifecycle data, are retained.
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

    /// Merges compact person information into this record.
    ///
    /// Overwrites `id`, `name`, `gender`, `birthday`, `credential`, `test`, and `delete_time`.
    /// It also replaces the entire `contact` value with a new projection containing only mobile
    /// and email. Every other field, including the photograph and organization, is retained.
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

    /// Merges a saved order consignee into this record.
    ///
    /// Overwrites `id`, `name`, and `credential`, then replaces the entire `contact` value with
    /// the consignee's mobile, email, and address. Fields not listed here are retained.
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

    /// Merges an order-buyer projection into this record.
    ///
    /// Overwrites `id`, `name`, `credential`, `gender`, and `birthday`. It replaces the entire
    /// `contact` value with a new projection containing only mobile and email; all other fields
    /// remain unchanged.
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

    /// Applies compact person information using the same replacement rules as [`Self::assign_info`].
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
    /// When `other.person_id()` returns `Some`, compares it with `self.id.value() as i64`,
    /// including the default identifier. Credentials are compared only when
    /// `other.person_id()` returns `None`; a matching credential pair returns `true`, and missing
    /// credentials return `false`.
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
