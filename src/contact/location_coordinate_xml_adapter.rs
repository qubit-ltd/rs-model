// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML-wire adapter for coordinates.

use bigdecimal::BigDecimal;

use crate::contact::ContactCodecError;
use crate::contact::LocationCoordinateCodec;

/// Marshals and unmarshals optional coordinates as XML text values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocationCoordinateXmlAdapter;

impl LocationCoordinateXmlAdapter {
    /// Decodes an optional XML text value.
    pub fn unmarshal(
        value: Option<&str>,
    ) -> Result<Option<BigDecimal>, ContactCodecError> {
        LocationCoordinateCodec::decode(value)
    }

    /// Encodes an optional coordinate as XML text.
    #[must_use]
    pub fn marshal(value: Option<&BigDecimal>) -> Option<String> {
        LocationCoordinateCodec::encode(value)
    }
}
