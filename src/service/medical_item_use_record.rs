// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical service usage records.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::organization::EmployeeInfo;
use crate::person::PersonInfo;
use crate::service::MedicalItem;
use crate::service::MedicalPackage;

/// Records a patient's use of one item from a medical service package.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalItemUseRecord {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Medical package from which the item was consumed.
    pub medical_package: MedicalPackage,

    /// Medical service item that was consumed.
    pub medical_item: MedicalItem,

    /// Persisted identifier of the user's item entitlement.
    #[model(opaque)]
    pub user_medical_item_id: Id,

    /// Patient who used the service.
    pub patient: PersonInfo,

    /// Doctor who provided the service.
    pub doctor: EmployeeInfo,
}
