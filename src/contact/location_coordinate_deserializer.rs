// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON-wire deserializer for coordinates.

use bigdecimal::BigDecimal;

use crate::contact::{
    ContactCodecError,
    LocationCoordinateCodec,
};

/// Deserializes a coordinate from the source decimal representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocationCoordinateDeserializer;

impl LocationCoordinateDeserializer {
    /// Deserializes a non-null coordinate.
    pub fn deserialize(value: &str) -> Result<BigDecimal, ContactCodecError> {
        LocationCoordinateCodec::decode(Some(value))?
            .ok_or(ContactCodecError::InvalidCoordinate)
    }
}
