// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-issuing locations.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{commons::State, mixin::StatefulInfo};

/// A coded invoice-issuing location within an organization.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct InvoicePlace {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Application that owns this issuing location.
    pub app: StatefulInfo,
    /// Organization that owns this issuing location.
    pub organization: StatefulInfo,
    /// Issuing-location code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Issuing-location name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Current lifecycle state.
    pub state: State,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
