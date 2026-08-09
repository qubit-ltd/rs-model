// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Domain models exchanged by clinical records, prescription workflows, and
//! medical-insurance settlement processing.

mod clinic_info;
mod diagnosis;
mod disease;
mod dosage;
mod drug;
mod drug_info;
mod drug_product;
mod emergent_clinic_info;
mod examination_info;
mod his_info;
mod hospital_drugstore;
mod hospitalization_info;
mod medical_invoice_type;
mod medical_payment;
mod medical_settlement;
mod medical_settlement_item;
mod medical_type;
mod medicare_item_type;
mod medicare_type;
mod patient;
mod patient_info;
#[allow(clippy::module_inception)]
mod prescription;
mod prescription_action;
mod prescription_action_params;
mod prescription_content;
mod prescription_item;
mod prescription_order_request;
mod prescription_status;
mod registration_info;
mod specific_clinic_info;

pub use clinic_info::ClinicInfo;
pub use diagnosis::Diagnosis;
pub use disease::Disease;
pub use dosage::Dosage;
pub use drug::Drug;
pub use drug_info::DrugInfo;
pub use drug_product::DrugProduct;
pub use emergent_clinic_info::EmergentClinicInfo;
pub use examination_info::ExaminationInfo;
pub use his_info::HisInfo;
pub use hospital_drugstore::HospitalDrugstore;
pub use hospitalization_info::HospitalizationInfo;
pub use medical_invoice_type::MedicalInvoiceType;
pub use medical_payment::MedicalPayment;
pub use medical_settlement::MedicalSettlement;
pub use medical_settlement_item::MedicalSettlementItem;
pub use medical_type::MedicalType;
pub use medicare_item_type::MedicareItemType;
pub use medicare_type::MedicareType;
pub use patient::Patient;
pub use patient_info::PatientInfo;
pub use prescription::Prescription;
pub use prescription_action::PrescriptionAction;
pub use prescription_action_params::PrescriptionActionParams;
pub use prescription_content::PrescriptionContent;
pub use prescription_item::PrescriptionItem;
pub use prescription_order_request::PrescriptionOrderRequest;
pub use prescription_status::PrescriptionStatus;
pub use registration_info::RegistrationInfo;
pub use specific_clinic_info::SpecificClinicInfo;
