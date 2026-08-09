// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Product-level rules that restrict purchasers, recipients, sources, and frequency.

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::product::PersonConstraint;

/// Independently optional purchase restrictions applied before an order is accepted.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProductConstraint {
    /// Reference date for age evaluation, or `None` to use the consuming service's default.
    pub age_epoch: Option<NaiveDate>,

    /// Adult threshold in whole years, or `None` when the default threshold applies.
    pub adult_age: Option<i32>,

    /// Whether the buyer must also be the client, or `None` when either arrangement is allowed.
    pub self_only: Option<bool>,

    /// Eligibility requirements for the buyer, or `None` when the buyer is unrestricted.
    pub buyer: Option<PersonConstraint>,

    /// Eligibility requirements for the recipient, or `None` when the recipient is unrestricted.
    pub client: Option<PersonConstraint>,

    /// Allowed source identifiers; `None` means unrestricted.
    pub sources: Option<Vec<String>>,

    /// Per-buyer purchase cap, or `None` when no cap is imposed.
    pub limit_for_buyer: Option<i32>,

    /// Per-client purchase cap, or `None` when no cap is imposed.
    pub limit_for_client: Option<i32>,
}
