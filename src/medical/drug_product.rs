// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Links between clinical drug references and products offered for sale.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::medical::DrugInfo;
use crate::product::ProductInfo;

/// Maps the prescribed-drug snapshot to the corresponding purchasable product.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DrugProduct {
    /// Medical drug information.
    pub drug: DrugInfo,

    /// Sellable product information.
    pub product: ProductInfo,
}
