// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Administrative-region hierarchy values.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// Distinguishes each level of the source administrative hierarchy.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Region {
    /// A country.
    Country,
    /// A province within a country.
    Province,
    /// A city within a province.
    City,
    /// A district within a city.
    District,
    /// A street within a district.
    Street,
}

impl Region {
    /// Returns this region's immediate parent level, if it has one.
    #[must_use]
    pub const fn parent(self) -> Option<Self> {
        match self {
            Self::Country => None,
            Self::Province => Some(Self::Country),
            Self::City => Some(Self::Province),
            Self::District => Some(Self::City),
            Self::Street => Some(Self::District),
        }
    }
}
