// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON-wire serializer for coordinates.

use bigdecimal::BigDecimal;

use crate::contact::LocationCoordinateCodec;

/// Serializes a coordinate using the source six-decimal representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocationCoordinateSerializer;

impl LocationCoordinateSerializer {
    /// Serializes a non-null coordinate.
    #[must_use]
    pub fn serialize(value: &BigDecimal) -> String {
        LocationCoordinateCodec::encode(Some(value)).expect("present coordinate")
    }
}
