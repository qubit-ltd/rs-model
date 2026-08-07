// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Drug-to-product mappings.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::medical::DrugInfo;
use crate::product::ProductInfo;

/// Associates a medical drug snapshot with a sellable product snapshot.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DrugProduct {
    /// Medical drug information.
    pub drug: DrugInfo,

    /// Sellable product information.
    pub product: ProductInfo,
}
