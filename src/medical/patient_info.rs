// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Patient snapshot data embedded in settlement and clinical payloads.

use chrono::NaiveDate;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Phone;
use crate::person::Gender;

/// The identifying patient details needed by a medical record without the full
/// patient profile.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct PatientInfo {
    /// Optional persisted patient identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique patient code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Patient code within the owning hospital.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: String,

    /// Patient's legal name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Patient's gender.
    pub gender: Gender,

    /// Patient's date of birth.
    pub birthday: NaiveDate,

    /// Patient's verified identity credential.
    pub credential: CredentialInfo,

    /// Patient's mobile telephone number.
    pub mobile: Phone,
}
