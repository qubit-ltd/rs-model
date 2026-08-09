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

/// Employment seniority or professional-title band.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobTitle {
    /// Self-employed or freelance worker.
    Freelance,
    /// Individual contributor without a title band.
    Employee,
    /// Junior professional title.
    JuniorTitle,
    /// Intermediate professional title.
    MiddleTitle,
    /// Senior professional title.
    SeniorTitle,
    /// First-line manager.
    JuniorManager,
    /// Middle manager.
    MiddleManager,
    /// Senior manager.
    SeniorManager,
    /// Business owner.
    Owner,
}
