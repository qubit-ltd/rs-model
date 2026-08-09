// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Included service entitlements within a medical package.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::service::MedicalItem;

/// A medical service item and the number of uses granted by a package.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalPackageItem {
    /// Identifier of the medical package that grants this entitlement.
    #[model(opaque)]
    pub package_id: Id,

    /// Medical service item included in the package.
    pub item: MedicalItem,

    /// Number of times the holder may use this item.
    pub count: i32,
}
