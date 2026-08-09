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

/// Annual income band in the source system's currency unit (thousands).
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Incoming {
    /// Below 25k per year.
    Annual25kBelow,
    /// 25k to 50k per year.
    Annual25k50k,
    /// 50k to 100k per year.
    Annual50k100k,
    /// 100k to 150k per year.
    Annual100k150k,
    /// 150k to 200k per year.
    Annual150k200k,
    /// 200k to 300k per year.
    Annual200k300k,
    /// 300k to 400k per year.
    Annual300k400k,
    /// 400k to 500k per year.
    Annual400k500k,
    /// 500k to 800k per year.
    Annual500k800k,
    /// 800k to 1,000k per year.
    Annual800k1000k,
    /// 1,000k to 5,000k per year.
    Annual1000k5000k,
    /// 5,000k to 10,000k per year.
    Annual5000k10000k,
    /// At least 10,000k per year.
    Annual10000kAbove,
}
