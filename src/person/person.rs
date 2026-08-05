//! Complete person records.

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use qubit_mixin::{Info, InfoWithEntity};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::CredentialInfo;
use crate::contact::Contact;
use crate::medical::MedicareType;
use crate::mixin::StatefulInfo;
use crate::person::{
    Blood, Education, Ethnic, Gender, Incoming, Industry, JobTitle, Marriage, PersonInfo, Politics,
    Religion, SexOrientation,
};
use crate::upload::Attachment;

/// A person's complete demographic, contact, and administrative record.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Person {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Optional data-source information.
    #[model(opaque)]
    pub source: Option<InfoWithEntity>,
    /// Optional classification information.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,
    /// Real name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional globally unique registered user name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub username: Option<String>,
    /// Optional gender.
    pub gender: Option<Gender>,
    /// Optional birth date.
    pub birthday: Option<NaiveDate>,
    /// Optional birth time.
    pub birth_time: Option<NaiveTime>,
    /// Optional birth-country information.
    #[model(opaque)]
    pub birth_country: Option<Info>,
    /// Optional birth-province information.
    #[model(opaque)]
    pub birth_province: Option<Info>,
    /// Optional birth-city information.
    #[model(opaque)]
    pub birth_city: Option<Info>,
    /// Optional identity credential.
    pub credential: Option<CredentialInfo>,
    /// Whether the person has medical insurance.
    pub has_medicare: Option<bool>,
    /// Optional medical-insurance classification.
    pub medicare_type: Option<MedicareType>,
    /// Optional medical-insurance card.
    pub medicare_card: Option<CredentialInfo>,
    /// Optional medical-insurance city.
    #[model(opaque)]
    pub medicare_city: Option<Info>,
    /// Whether the person has social security.
    pub has_social_security: Option<bool>,
    /// Optional social-security card.
    pub social_security_card: Option<CredentialInfo>,
    /// Optional social-security city.
    #[model(opaque)]
    pub social_security_city: Option<Info>,
    /// Optional contact details.
    #[redact(nested)]
    pub contact: Option<Contact>,
    /// Optional guardian information.
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
    pub photo: Option<Attachment>,
    /// Optional comment.
    pub comment: Option<String>,
    /// Whether this is test data.
    pub test: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
