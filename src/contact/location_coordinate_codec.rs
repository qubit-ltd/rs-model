// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Decimal codec and normalization for location coordinates.

use bigdecimal::BigDecimal;
use bigdecimal::RoundingMode;
use std::str::FromStr;

use crate::contact::ContactCodecError;

/// Encodes, decodes, and normalizes decimal-degree coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocationCoordinateCodec;

impl LocationCoordinateCodec {
    /// Number of digits retained after the decimal point.
    pub const SCALE: i64 = 6;

    /// Normalizes a coordinate to six decimal places and the inclusive range
    /// `[-180, 180]`.
    #[must_use]
    pub fn normalize(value: Option<BigDecimal>) -> Option<BigDecimal> {
        Self::normalize_with_precision(value, Self::SCALE)
    }

    /// Normalizes a coordinate using the requested number of fractional digits.
    #[must_use]
    pub fn normalize_with_precision(
        value: Option<BigDecimal>,
        precision: i64,
    ) -> Option<BigDecimal> {
        value.map(|value| {
            let round_degree = BigDecimal::from(360);
            let minimum = BigDecimal::from(-180);
            let maximum = BigDecimal::from(180);
            let mut normalized =
                value.with_scale_round(precision, RoundingMode::HalfUp) % &round_degree;
            if normalized < minimum {
                normalized += &round_degree;
            } else if normalized > maximum {
                normalized -= &round_degree;
            }
            normalized.with_scale(precision)
        })
    }

    /// Decodes a decimal coordinate, treating null as absent.
    pub fn decode(source: Option<&str>) -> Result<Option<BigDecimal>, ContactCodecError> {
        source
            .map(|source| {
                BigDecimal::from_str(source.trim())
                    .map_err(|_| ContactCodecError::InvalidCoordinate)
                    .map(|value| Self::normalize(Some(value)).expect("present coordinate"))
            })
            .transpose()
    }

    /// Encodes a coordinate with the source precision.
    #[must_use]
    pub fn encode(source: Option<&BigDecimal>) -> Option<String> {
        source
            .and_then(|value| Self::normalize(Some(value.clone())))
            .map(|value| value.to_string())
    }
}
