// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Source-domain Incoming classification.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Incoming {
    /// Source variant `ANNUAL_25K_BELOW`.
    Annual25kBelow,
    /// Source variant `ANNUAL_25K_50K`.
    Annual25k50k,
    /// Source variant `ANNUAL_50K_100K`.
    Annual50k100k,
    /// Source variant `ANNUAL_100K_150K`.
    Annual100k150k,
    /// Source variant `ANNUAL_150K_200K`.
    Annual150k200k,
    /// Source variant `ANNUAL_200K_300K`.
    Annual200k300k,
    /// Source variant `ANNUAL_300K_400K`.
    Annual300k400k,
    /// Source variant `ANNUAL_400K_500K`.
    Annual400k500k,
    /// Source variant `ANNUAL_500K_800K`.
    Annual500k800k,
    /// Source variant `ANNUAL_800K_1000K`.
    Annual800k1000k,
    /// Source variant `ANNUAL_1000K_5000K`.
    Annual1000k5000k,
    /// Source variant `ANNUAL_5000K_10000K`.
    Annual5000k10000k,
    /// Source variant `ANNUAL_10000K_ABOVE`.
    Annual10000kAbove,
}
