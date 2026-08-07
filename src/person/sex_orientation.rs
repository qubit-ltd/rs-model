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

/// Source-domain SexOrientation classification.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SexOrientation {
    /// Source variant `HETEROSEXUAL`.
    Heterosexual,
    /// Source variant `HOMOSEXUAL`.
    Homosexual,
    /// Source variant `BISEXUAL`.
    Bisexual,
    /// Source variant `SECRECY`.
    Secrecy,
}
