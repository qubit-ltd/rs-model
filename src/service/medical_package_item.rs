// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical package line items.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::service::MedicalItem;

/// A medical service item and its quantity within a package.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct MedicalPackageItem {
    /// Persisted identifier of the owning medical package.
    pub package_id: i64,
    /// Included medical service item.
    pub item: MedicalItem,
    /// Number of uses included in the package.
    pub count: i32,
}
