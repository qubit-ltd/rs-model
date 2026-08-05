// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared enumerations from the Java commons model package.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies a credential document type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialType {
    /// Chinese identity card.
    IdentityCard,
    /// Household residence booklet.
    ResidenceBooklet,
    /// Passport.
    Passport,
    /// Military officer card.
    OfficerCard,
    /// Driving license.
    DrivingCard,
    /// Hong Kong or Macao mainland return permit.
    HongkongMacaoReturnPermit,
    /// Taiwan mainland return permit.
    TaiwanReturnPermit,
    /// Police identification card.
    PoliceCard,
    /// Hong Kong passport or identification.
    HongkongPassport,
    /// Macao passport or identification.
    MacaoPassport,
    /// Taiwan passport or identification.
    TaiwanPassport,
    /// Foreigner permanent-residence permit.
    ForeignerPermanentResidencePermit,
    /// Hong Kong, Macao, or Taiwan residence permit.
    HongkongMacaoTaiwanResidencePermit,
    /// Birth certificate.
    BirthCertificate,
    /// Social-security card.
    SocialSecurityCard,
    /// Medical-insurance card.
    MedicareCard,
    /// Employee card.
    EmployeeCard,
    /// Practising certificate.
    PractisingCertificate,
    /// Professional-title certificate.
    TitleCertificate,
    /// Business license.
    BusinessLicense,
    /// Organization code certificate.
    OrganizationCode,
    /// Other credential type.
    Other,
}

impl CredentialType {
    /// Returns the stable Java credential-type code.
    ///
    /// # Returns
    ///
    /// The two-digit credential-type code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::IdentityCard => "01",
            Self::ResidenceBooklet => "02",
            Self::Passport => "03",
            Self::OfficerCard => "04",
            Self::DrivingCard => "05",
            Self::HongkongMacaoReturnPermit => "06",
            Self::TaiwanReturnPermit => "07",
            Self::PoliceCard => "14",
            Self::HongkongPassport => "16",
            Self::MacaoPassport => "17",
            Self::TaiwanPassport => "18",
            Self::ForeignerPermanentResidencePermit => "19",
            Self::HongkongMacaoTaiwanResidencePermit => "20",
            Self::BirthCertificate => "21",
            Self::SocialSecurityCard => "31",
            Self::MedicareCard => "32",
            Self::EmployeeCard => "50",
            Self::PractisingCertificate => "51",
            Self::TitleCertificate => "52",
            Self::BusinessLicense => "70",
            Self::OrganizationCode => "71",
            Self::Other => "99",
        }
    }
}

/// Identifies one supported currency.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    /// Chinese yuan.
    Cny,
    /// Hong Kong dollar.
    Hkd,
    /// New Taiwan dollar.
    Twd,
    /// United States dollar.
    Usd,
    /// Euro.
    Eur,
    /// Pound sterling.
    Gbp,
    /// Japanese yen.
    Jpy,
    /// Virtual currency.
    Virtual,
}

/// Describes which calendar dates a schedule accepts.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayType {
    /// Any date.
    Any,
    /// Working days.
    WorkingDay,
    /// Holidays.
    Holiday,
    /// Weekends.
    Weekend,
}

/// Describes a kinship relationship.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Kinship {
    /// The subject themself.
    Self_,
    /// Parent relationship.
    Parent,
    /// Spouse relationship.
    Spouse,
    /// Child relationship.
    Child,
    /// Other relationship.
    Other,
}

/// Identifies the type of a failed message-queue task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MqType {
    /// A produced message.
    Produce,
}

/// Describes the lifecycle of a request.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
    /// Newly created.
    Created,
    /// Submitted for processing.
    Submitted,
    /// Waiting for processing.
    Pending,
    /// Being processed.
    Processing,
    /// Processing failed.
    Failed,
    /// Cancelled before completion.
    Cancelled,
    /// Completed successfully.
    Completed,
}

impl RequestStatus {
    /// Returns the Java-compatible serialized name.
    ///
    /// # Returns
    ///
    /// The screaming-snake-case status name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "CREATED",
            Self::Submitted => "SUBMITTED",
            Self::Pending => "PENDING",
            Self::Processing => "PROCESSING",
            Self::Failed => "FAILED",
            Self::Cancelled => "CANCELLED",
            Self::Completed => "COMPLETED",
        }
    }
}

/// Describes a verification lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifyState {
    /// Verification has not started.
    None,
    /// Verification is in progress.
    Verifying,
    /// Verification succeeded.
    Valid,
    /// Verification failed.
    Invalid,
}

impl VerifyState {
    /// Returns the Java-compatible serialized name.
    ///
    /// # Returns
    /// The screaming-snake-case verification-state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Verifying => "VERIFYING",
            Self::Valid => "VALID",
            Self::Invalid => "INVALID",
        }
    }
}
