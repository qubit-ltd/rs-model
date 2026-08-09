// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Regulatory and clinical reference data for medicines in the drug catalog.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::commons::DictEntryInfo;

/// A complete medicine catalog entry, including regulatory, dispensing, and
/// administration information.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Drug {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique internal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Generic name listed in the national drug standard.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,

    /// Optional drug category.
    #[model(opaque)]
    pub category: Option<Info>,

    /// Proprietary name approved for use by a particular manufacturer.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub product_name: String,

    /// English rendering of the generic drug name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub english_name: Option<String>,

    /// Hanyu Pinyin rendering of the generic drug name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub pinyin_name: Option<String>,

    /// Optional chemical name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub chemical_name: Option<String>,

    /// Strength and package specification that distinguishes the catalog item.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,

    /// Dictionary entry for the prepared pharmaceutical form used for treatment
    /// or prevention.
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

    /// Physical characteristics such as colour, appearance, or taste.
    pub characteristics: Option<String>,

    /// Optional composition description.
    pub composition: Option<String>,

    /// Conditions for which this medicine is indicated.
    pub indications: Option<String>,

    /// Recommended method and quantity for taking the medicine.
    pub dosage: Option<String>,

    /// Known adverse symptoms that may occur while taking the medicine.
    pub adverse_reaction: Option<String>,

    /// Optional contraindications.
    pub contraindications: Option<String>,

    /// Optional precautions.
    pub precautions: Option<String>,

    /// Optional storage instructions.
    pub storage: Option<String>,

    /// Domestic manufacturing approval number issued by the drug regulator.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub license_number: Option<String>,

    /// Optional license approval date.
    pub license_date: Option<NaiveDate>,

    /// Certificate number issued after regulatory review of an imported drug.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub import_number: Option<String>,

    /// Registration-certificate number for a medicine produced in Hong Kong,
    /// Macao, or Taiwan.
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

    /// ISO-8601 period for which the medicine remains within shelf life.
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
