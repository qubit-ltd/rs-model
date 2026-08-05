// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! China administrative-data and resident identity-card utilities.

mod china_cities;
mod china_districts;
mod china_provinces;
mod identity_card_utils;

pub use china_cities::ChinaCities;
pub use china_districts::ChinaDistricts;
pub use china_provinces::ChinaProvinces;
pub use identity_card_utils::IdentityCardUtils;
