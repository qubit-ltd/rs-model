// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Mobile device inventory and telemetry classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Radio-access technology reported for a SIM card's mobile data connection.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataNetworkType {
    /// General Packet Radio Service (2G).
    Gprs,
    /// Code Division Multiple Access (2G).
    Cdma,
    /// Enhanced Data rates for GSM Evolution (2G).
    Edge,
    /// CDMA2000 1xRTT (2G).
    OneXRtt,
    /// Integrated Digital Enhanced Network (2G).
    Iden,
    /// Global System for Mobile Communications (2G).
    Gsm,
    /// TD-SCDMA (3G).
    TdScdma,
    /// CDMA2000 (3G).
    Cdma2000,
    /// EV-DO revision A (3G).
    EvdoA,
    /// Universal Mobile Telecommunications System (3G).
    Umts,
    /// EV-DO revision 0 (3G).
    Evdo0,
    /// High-Speed Downlink Packet Access (3G).
    Hsdpa,
    /// High-Speed Uplink Packet Access (3G).
    Hsupa,
    /// High-Speed Packet Access (3G).
    Hspa,
    /// EV-DO revision B (3G).
    EvdoB,
    /// Evolved High Rate Packet Data (3G).
    Ehrpd,
    /// HSPA+ (3G).
    Hspap,
    /// IWLAN; reported as a 3G-era technology by the source model.
    Iwlan,
    /// Long-Term Evolution (4G).
    Lte,
    /// New Radio (5G).
    Nr,
    /// The radio technology was not reported or is not recognized.
    Unknown,
}

impl DataNetworkType {
    /// Returns the source model's cellular-generation number: 2, 3, 4, or 5.
    ///
    /// Returns `0` when the technology is [`Self::Unknown`].
    #[must_use]
    pub const fn generation(self) -> i32 {
        match self {
            Self::Lte => 4,
            Self::Nr => 5,
            Self::Unknown => 0,
            Self::Gprs | Self::Cdma | Self::Edge | Self::OneXRtt | Self::Iden | Self::Gsm => 2,
            _ => 3,
        }
    }
}
