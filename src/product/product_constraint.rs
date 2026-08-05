// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Purchase constraints for a product.

use chrono::NaiveDate;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::product::PersonConstraint;

/// Optional rules that constrain who can purchase a product and how often.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct ProductConstraint {
    /// Date against which an age restriction is evaluated.
    pub age_epoch: Option<NaiveDate>,
    /// Age in years at which a person is considered an adult.
    pub adult_age: Option<i32>,
    /// Whether a buyer may purchase only for themself.
    pub self_only: Option<bool>,
    /// Restrictions applied to the buyer.
    pub buyer: Option<PersonConstraint>,
    /// Restrictions applied to the client receiving the product.
    pub client: Option<PersonConstraint>,
    /// Allowed source identifiers; `None` means unrestricted.
    pub sources: Option<Vec<String>>,
    /// Maximum number of purchases allowed for one buyer.
    pub limit_for_buyer: Option<i32>,
    /// Maximum number of purchases allowed for one client.
    pub limit_for_client: Option<i32>,
}
