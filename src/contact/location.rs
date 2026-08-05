//! Geographic location values.

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::contact::CoordinateSystem;

/// A geographic location expressed as longitude, latitude, and optional altitude.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
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
