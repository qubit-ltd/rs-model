// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Records failures and the last authorization time.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct AuthorizeRecord {
    /// Consecutive authorization failures.
    pub failures: Option<i32>,

    /// Optional last authorization timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub time: Option<DateTime<Utc>>,
}
