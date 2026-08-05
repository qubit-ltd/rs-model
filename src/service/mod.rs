// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical-service entitlements, packages, and usage records.

mod employee_medical_item;
mod medical_item;
mod medical_item_use_record;
mod medical_package;
mod medical_package_item;
mod user_medical_item;
mod user_medical_package;
mod user_service_state;

pub use employee_medical_item::EmployeeMedicalItem;
pub use medical_item::MedicalItem;
pub use medical_item_use_record::MedicalItemUseRecord;
pub use medical_package::MedicalPackage;
pub use medical_package_item::MedicalPackageItem;
pub use user_medical_item::UserMedicalItem;
pub use user_medical_package::UserMedicalPackage;
pub use user_service_state::UserServiceState;
