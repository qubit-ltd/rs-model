// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical service usage records.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    organization::EmployeeInfo,
    person::PersonInfo,
    service::{MedicalItem, MedicalPackage},
};

/// Records a patient's use of one item from a medical service package.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct MedicalItemUseRecord {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Medical package from which the item was consumed.
    pub medical_package: MedicalPackage,
    /// Medical service item that was consumed.
    pub medical_item: MedicalItem,
    /// Persisted identifier of the user's item entitlement.
    pub user_medical_item_id: i64,
    /// Patient who used the service.
    pub patient: PersonInfo,
    /// Doctor who provided the service.
    pub doctor: EmployeeInfo,
}
