// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Complete drug catalog records.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::DictEntryInfo;

/// A pharmaceutical catalog record with regulatory and administration data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Drug {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Globally unique internal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Generic drug name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,

    /// Optional drug category.
    #[model(opaque)]
    pub category: Option<Info>,

    /// Approved product name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub product_name: String,

    /// Optional English generic name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub english_name: Option<String>,

    /// Optional pinyin generic name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub pinyin_name: Option<String>,

    /// Optional chemical name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub chemical_name: Option<String>,

    /// Drug specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,

    /// Dosage-form dictionary entry.
    pub dosage_form: DictEntryInfo,

    /// Optional administration-route dictionary entry.
    pub administration_route: Option<DictEntryInfo>,

    /// Optional administration-frequency dictionary entry.
    pub frequency: Option<DictEntryInfo>,

    /// Standard unit.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,

    /// Packaging description.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub packaging: String,

    /// Whether this is an essential medicine.
    pub basic: bool,

    /// Whether this is an over-the-counter medicine.
    pub otc: bool,

    /// Whether this is an antibiotic.
    pub antibiotics: bool,

    /// Whether use is restricted.
    pub restricted: bool,

    /// Whether this is a single or compound herbal medicine.
    pub herbal_compound: bool,

    /// Whether this is a specially supplied medicine.
    pub special: bool,

    /// Optional physical-characteristics description.
    pub characteristics: Option<String>,

    /// Optional composition description.
    pub composition: Option<String>,

    /// Optional indications.
    pub indications: Option<String>,

    /// Optional dosage instructions.
    pub dosage: Option<String>,

    /// Optional adverse-reaction description.
    pub adverse_reaction: Option<String>,

    /// Optional contraindications.
    pub contraindications: Option<String>,

    /// Optional precautions.
    pub precautions: Option<String>,

    /// Optional storage instructions.
    pub storage: Option<String>,

    /// Optional domestic production-license number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub license_number: Option<String>,

    /// Optional license approval date.
    pub license_date: Option<NaiveDate>,

    /// Optional imported-drug registration number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub import_number: Option<String>,

    /// Optional Hong Kong, Macao, or Taiwan registration number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub registration_number: Option<String>,

    /// Optional brand.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub brand: Option<String>,

    /// Optional place of origin.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub origin: Option<String>,

    /// Optional manufacturer information.
    #[model(opaque)]
    pub manufacturer: Option<Info>,

    /// Optional drug-classification dictionary entry.
    pub classification: Option<DictEntryInfo>,

    /// Optional ISO-8601 shelf-life period.
    #[model(opaque)]
    pub shelf_life: Option<String>,

    /// Optional remark.
    pub comment: Option<String>,

    /// Whether this drug is predefined reference data.
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
