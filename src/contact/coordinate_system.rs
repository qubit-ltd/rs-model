// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Geographic coordinate-system values.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the coordinate system used by a geographic location.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CoordinateSystem {
    /// Global WGS-84 coordinates used by GPS.
    Wgs84,
    /// China GCJ-02 coordinates.
    Gcj02,
    /// Baidu BD-09 coordinates.
    Bd09,
}

impl CoordinateSystem {
    /// Returns the stable source-domain code for this coordinate system.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Wgs84 => "WGS-84",
            Self::Gcj02 => "GCJ-02",
            Self::Bd09 => "BD-09",
        }
    }

    /// Returns the English source-domain description for this coordinate
    /// system.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Wgs84 => "World Geodetic System 1984",
            Self::Gcj02 => "Mars Coordinate System",
            Self::Bd09 => "Baidu Coordinate System",
        }
    }
}
