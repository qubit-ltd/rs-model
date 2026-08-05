// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Device classification types.

#[allow(unused_imports)]
use super::{
    DeviceType,
    SimCardStatus,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Mobile data-radio protocol classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataNetworkType {
    /// The Gprs classification.
    Gprs,
    /// The Cdma classification.
    Cdma,
    /// The Edge classification.
    Edge,
    /// The OneXRtt classification.
    OneXRtt,
    /// The Iden classification.
    Iden,
    /// The Gsm classification.
    Gsm,
    /// The TdScdma classification.
    TdScdma,
    /// The Cdma2000 classification.
    Cdma2000,
    /// The EvdoA classification.
    EvdoA,
    /// The Umts classification.
    Umts,
    /// The Evdo0 classification.
    Evdo0,
    /// The Hsdpa classification.
    Hsdpa,
    /// The Hsupa classification.
    Hsupa,
    /// The Hspa classification.
    Hspa,
    /// The EvdoB classification.
    EvdoB,
    /// The Ehrpd classification.
    Ehrpd,
    /// The Hspap classification.
    Hspap,
    /// The Iwlan classification.
    Iwlan,
    /// The Lte classification.
    Lte,
    /// The Nr classification.
    Nr,
    /// The Unknown classification.
    Unknown,
}

impl DataNetworkType {
    /// Returns the cellular-network generation represented by this value.
    #[must_use]
    pub const fn generation(self) -> i32 {
        match self {
            Self::Lte => 4,
            Self::Nr => 5,
            Self::Unknown => 0,
            Self::Gprs
            | Self::Cdma
            | Self::Edge
            | Self::OneXRtt
            | Self::Iden
            | Self::Gsm => 2,
            _ => 3,
        }
    }
}
