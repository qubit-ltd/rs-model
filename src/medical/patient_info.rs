// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight patient information.

use chrono::NaiveDate;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{commons::CredentialInfo, contact::Phone, person::Gender};

/// A compact patient snapshot embedded in medical records.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct PatientInfo {
    /// Optional persisted patient identifier.
    #[model(identifier)]
    pub id: Option<i64>,

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
