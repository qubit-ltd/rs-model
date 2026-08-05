// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight drug information.

use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::commons::DictEntryInfo;

/// A compact drug snapshot embedded in prescriptions and products.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DrugInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Globally unique drug code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Generic drug name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,
    /// Approved product name.
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
    /// Whether this is a single or compound herbal medicine.
    pub herbal_compound: bool,
    /// Optional brand.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub brand: Option<String>,
    /// Optional place of origin.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub origin: Option<String>,
    /// Optional manufacturer information.
    #[model(opaque)]
    pub manufacturer: Option<Info>,
}
