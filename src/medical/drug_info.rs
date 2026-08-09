// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Drug snapshots carried in prescriptions and product mappings.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::commons::DictEntryInfo;

/// A portable drug snapshot for prescriptions and commerce records, containing
/// the display, packaging, and dispensing flags needed without the full catalog
/// monograph.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DrugInfo {
    /// Typed identifier carried by this drug snapshot when available upstream.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique catalogue code for the referenced drug.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Generic drug name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,

    /// Manufacturer-specific proprietary product name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub product_name: String,

    /// Drug specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,

    /// Dosage-form dictionary entry.
    pub dosage_form: DictEntryInfo,

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

    /// Whether the medicine is classified as a single or compound herbal drug.
    pub herbal_compound: bool,

    /// Brand name carried from the catalogue, absent when none is recorded.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub brand: Option<String>,

    /// Place of origin carried from the catalogue, absent when none is recorded.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub origin: Option<String>,

    /// Manufacturer identity carried from the catalogue, absent when unknown.
    #[model(opaque)]
    pub manufacturer: Option<Info>,
}
