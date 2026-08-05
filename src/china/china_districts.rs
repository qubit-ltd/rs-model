// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! China administrative-data markers and resident identity-card utilities.

#[allow(unused_imports)]
use super::{
    ChinaCities,
    ChinaProvinces,
    IdentityCardUtils,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Marker for the source China district dataset.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize,
)]
pub struct ChinaDistricts;
