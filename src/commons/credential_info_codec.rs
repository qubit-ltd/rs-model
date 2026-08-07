// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String codec for credential information.

use crate::commons::{CredentialInfo, CredentialInfoCodecError, CredentialType};

/// Converts credential information to and from its Java-compatible wire format.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CredentialInfoCodec;

impl CredentialInfoCodec {
    /// Decodes `[TYPE]-[NUMBER]`, treating null or blank input as absent.
    pub fn decode(
        source: Option<&str>,
    ) -> Result<Option<CredentialInfo>, CredentialInfoCodecError> {
        let Some(source) = source.map(str::trim).filter(|value| !value.is_empty()) else {
            return Ok(None);
        };
        let Some((type_name, number)) = source.split_once('-') else {
            return Err(CredentialInfoCodecError::InvalidFormat);
        };
        let credential_type = credential_type_for_name(type_name)?;
        Ok(Some(CredentialInfo {
            id: None,
            r#type: credential_type,
            number: number.to_owned(),
            verified: None,
        }))
    }

    /// Encodes credential information as `[TYPE]-[NUMBER]`.
    #[must_use]
    pub fn encode(source: Option<&CredentialInfo>) -> Option<String> {
        source.map(|credential| {
            format!(
                "{}-{}",
                credential_type_name(credential.r#type),
                credential.number
            )
        })
    }
}

/// Resolves the Java enumeration name without accepting aliases.
fn credential_type_for_name(name: &str) -> Result<CredentialType, CredentialInfoCodecError> {
    let value = match name {
        "IDENTITY_CARD" => CredentialType::IdentityCard,
        "RESIDENCE_BOOKLET" => CredentialType::ResidenceBooklet,
        "PASSPORT" => CredentialType::Passport,
        "OFFICER_CARD" => CredentialType::OfficerCard,
        "DRIVING_CARD" => CredentialType::DrivingCard,
        "HONGKONG_MACAO_RETURN_PERMIT" => CredentialType::HongkongMacaoReturnPermit,
        "TAIWAN_RETURN_PERMIT" => CredentialType::TaiwanReturnPermit,
        "POLICE_CARD" => CredentialType::PoliceCard,
        "HONGKONG_PASSPORT" => CredentialType::HongkongPassport,
        "MACAO_PASSPORT" => CredentialType::MacaoPassport,
        "TAIWAN_PASSPORT" => CredentialType::TaiwanPassport,
        "FOREIGNER_PERMANENT_RESIDENCE_PERMIT" => CredentialType::ForeignerPermanentResidencePermit,
        "HONGKONG_MACAO_TAIWAN_RESIDENCE_PERMIT" => {
            CredentialType::HongkongMacaoTaiwanResidencePermit
        }
        "BIRTH_CERTIFICATE" => CredentialType::BirthCertificate,
        "SOCIAL_SECURITY_CARD" => CredentialType::SocialSecurityCard,
        "MEDICARE_CARD" => CredentialType::MedicareCard,
        "EMPLOYEE_CARD" => CredentialType::EmployeeCard,
        "PRACTISING_CERTIFICATE" => CredentialType::PractisingCertificate,
        "TITLE_CERTIFICATE" => CredentialType::TitleCertificate,
        "BUSINESS_LICENSE" => CredentialType::BusinessLicense,
        "ORGANIZATION_CODE" => CredentialType::OrganizationCode,
        "OTHER" => CredentialType::Other,
        _ => {
            return Err(CredentialInfoCodecError::UnsupportedType(name.to_owned()));
        }
    };
    Ok(value)
}

/// Returns the stable Java enumeration name.
const fn credential_type_name(value: CredentialType) -> &'static str {
    match value {
        CredentialType::IdentityCard => "IDENTITY_CARD",
        CredentialType::ResidenceBooklet => "RESIDENCE_BOOKLET",
        CredentialType::Passport => "PASSPORT",
        CredentialType::OfficerCard => "OFFICER_CARD",
        CredentialType::DrivingCard => "DRIVING_CARD",
        CredentialType::HongkongMacaoReturnPermit => "HONGKONG_MACAO_RETURN_PERMIT",
        CredentialType::TaiwanReturnPermit => "TAIWAN_RETURN_PERMIT",
        CredentialType::PoliceCard => "POLICE_CARD",
        CredentialType::HongkongPassport => "HONGKONG_PASSPORT",
        CredentialType::MacaoPassport => "MACAO_PASSPORT",
        CredentialType::TaiwanPassport => "TAIWAN_PASSPORT",
        CredentialType::ForeignerPermanentResidencePermit => "FOREIGNER_PERMANENT_RESIDENCE_PERMIT",
        CredentialType::HongkongMacaoTaiwanResidencePermit => {
            "HONGKONG_MACAO_TAIWAN_RESIDENCE_PERMIT"
        }
        CredentialType::BirthCertificate => "BIRTH_CERTIFICATE",
        CredentialType::SocialSecurityCard => "SOCIAL_SECURITY_CARD",
        CredentialType::MedicareCard => "MEDICARE_CARD",
        CredentialType::EmployeeCard => "EMPLOYEE_CARD",
        CredentialType::PractisingCertificate => "PRACTISING_CERTIFICATE",
        CredentialType::TitleCertificate => "TITLE_CERTIFICATE",
        CredentialType::BusinessLicense => "BUSINESS_LICENSE",
        CredentialType::OrganizationCode => "ORGANIZATION_CODE",
        CredentialType::Other => "OTHER",
    }
}
