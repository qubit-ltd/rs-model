// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical records, prescriptions, settlements, and classifications.

mod disease;
mod dosage;
mod drug_info;
mod drug_product;
mod hospital_drugstore;
mod medical_invoice_type;
mod medical_type;
mod medicare_item_type;
mod medicare_type;
mod patient_info;
mod prescription_action;
mod prescription_status;

pub use disease::Disease;
pub use dosage::Dosage;
pub use drug_info::DrugInfo;
pub use drug_product::DrugProduct;
pub use hospital_drugstore::HospitalDrugstore;
pub use medical_invoice_type::MedicalInvoiceType;
pub use medical_type::MedicalType;
pub use medicare_item_type::MedicareItemType;
pub use medicare_type::MedicareType;
pub use patient_info::PatientInfo;
pub use prescription_action::PrescriptionAction;
pub use prescription_status::PrescriptionStatus;
