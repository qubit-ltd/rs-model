// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Geographic location values.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::CoordinateSystem;

/// A geographic location expressed as longitude, latitude, and optional
/// altitude.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Location {
    /// Longitude in decimal degrees, normalized to six decimal places.
    #[model(decimal(scale = 6))]
    pub longitude: BigDecimal,

    /// Latitude in decimal degrees, normalized to six decimal places.
    #[model(decimal(scale = 6))]
    pub latitude: BigDecimal,

    /// Optional altitude in meters, normalized to two decimal places.
    #[model(decimal(scale = 2))]
    pub altitude: Option<BigDecimal>,

    /// Optional coordinate system used by the numeric coordinates.
    pub coordinate_system: Option<CoordinateSystem>,
}
