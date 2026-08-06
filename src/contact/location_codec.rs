// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String codec for geographic locations.

use crate::contact::{
    ContactCodecError, CoordinateSystem, Location, LocationCoordinateCodec,
};

/// Converts locations to and from comma-separated longitude and latitude
/// values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocationCodec {
    /// Coordinate system assigned to decoded locations.
    coordinate_system: Option<CoordinateSystem>,
}

impl LocationCodec {
    /// Separator between longitude and latitude.
    pub const SPLITTER: char = ',';

    /// Creates a codec that assigns the supplied coordinate system when
    /// decoding.
    #[must_use]
    pub const fn new(coordinate_system: Option<CoordinateSystem>) -> Self {
        Self { coordinate_system }
    }

    /// Decodes a location, treating null or empty input as absent.
    pub fn decode(
        &self,
        source: Option<&str>,
    ) -> Result<Option<Location>, ContactCodecError> {
        let Some(source) = source.filter(|value| !value.is_empty()) else {
            return Ok(None);
        };
        let mut parts = source.split(Self::SPLITTER);
        let longitude = parts
            .next()
            .expect("nonempty input always has a first split component");
        let latitude =
            parts.next().ok_or(ContactCodecError::InvalidLocation)?;
        if parts.next().is_some() {
            return Err(ContactCodecError::InvalidLocation);
        }
        let longitude = LocationCoordinateCodec::decode(Some(longitude))?
            .expect("a present coordinate source never decodes to absence");
        let latitude = LocationCoordinateCodec::decode(Some(latitude))?
            .expect("a present coordinate source never decodes to absence");
        Ok(Some(Location {
            longitude,
            latitude,
            altitude: None,
            coordinate_system: self.coordinate_system,
        }))
    }

    /// Decodes a location and overrides the codec's coordinate system for this
    /// call.
    pub fn decode_with_coordinate_system(
        &self,
        source: Option<&str>,
        coordinate_system: Option<CoordinateSystem>,
    ) -> Result<Option<Location>, ContactCodecError> {
        let mut location = self.decode(source)?;
        if let Some(location) = &mut location {
            location.coordinate_system = coordinate_system;
        }
        Ok(location)
    }

    /// Encodes the longitude and latitude of a location.
    #[must_use]
    pub fn encode(&self, source: Option<&Location>) -> Option<String> {
        source.map(|location| {
            let longitude =
                LocationCoordinateCodec::encode(Some(&location.longitude))
                    .expect("present longitude");
            let latitude =
                LocationCoordinateCodec::encode(Some(&location.latitude))
                    .expect("present latitude");
            format!("{longitude}{}{latitude}", Self::SPLITTER)
        })
    }
}
