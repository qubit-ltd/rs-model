// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Product quality classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes whether a product is new or previously used.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Quality {
    /// A brand-new product.
    BrandNew,
    /// A previously used product.
    Used,
}
