// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Configurable rules that govern claim handling for an insurance product.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// A product-scoped key-value rule consumed by the claim-processing domain.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InsuranceProductRule {
    /// Typed identifier used when this product rule is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Insurance product information.
    #[model(opaque)]
    pub product: Info,

    /// Stable configuration key interpreted by the claim-processing product.
    pub key: String,

    /// Product-specific setting encoded for the associated rule key.
    pub value: String,

    /// Operator-facing explanation of the rule's business effect.
    pub description: String,

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
