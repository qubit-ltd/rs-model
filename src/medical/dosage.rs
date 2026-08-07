// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medication dosage instructions.

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::DictEntryInfo;

/// Instructions describing how and for how long to administer medication.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Dosage {
    /// Optional administration method or usage dictionary entry.
    pub usage: Option<DictEntryInfo>,

    /// Optional traditional-medicine decoction method.
    pub decoction: Option<DictEntryInfo>,

    /// Optional traditional-medicine therapeutic principle.
    #[model(text(min_chars = 1, max_chars = 512))]
    pub therapy: Option<String>,

    /// Amount administered each time.
    #[model(decimal(scale = 4))]
    pub amount: BigDecimal,

    /// Unit for each administered amount.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,

    /// Administration-frequency dictionary entry.
    pub frequency: DictEntryInfo,

    /// ISO-8601 duration for the medication course.
    #[model(opaque)]
    pub duration: String,

    /// Optional total amount administered during the course.
    #[model(decimal(scale = 4))]
    pub total_amount: Option<BigDecimal>,

    /// Optional number of traditional-medicine packets.
    pub pastes: Option<i32>,

    /// Optional administration precautions.
    pub precautions: Option<String>,
}
