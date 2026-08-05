// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical records, prescriptions, settlements, and classifications.

mod medical_invoice_type;
mod medical_type;
mod medicare_item_type;
mod medicare_type;
mod prescription_action;
mod prescription_status;

pub use medical_invoice_type::MedicalInvoiceType;
pub use medical_type::MedicalType;
pub use medicare_item_type::MedicareItemType;
pub use medicare_type::MedicareType;
pub use prescription_action::PrescriptionAction;
pub use prescription_status::PrescriptionStatus;
