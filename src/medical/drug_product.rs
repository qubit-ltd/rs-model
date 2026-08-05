// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Drug-to-product mappings.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{medical::DrugInfo, product::ProductInfo};

/// Associates a medical drug snapshot with a sellable product snapshot.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DrugProduct {
    /// Medical drug information.
    pub drug: DrugInfo,
    /// Sellable product information.
    pub product: ProductInfo,
}
