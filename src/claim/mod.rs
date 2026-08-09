// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Models for individual and employer-sponsored insurance claims, their
//! evidence, financial outcomes, and workflow state.

mod accident_reason;
pub mod enterprise;
#[allow(clippy::module_inception)]
mod insurance_claim;
mod insurance_claim_amount;
mod insurance_claim_event;
mod insurance_claim_invoice;
mod insurance_claim_invoice_cost;
mod insurance_claim_invoice_status;
mod insurance_claim_invoice_type;
mod insurance_claim_medical;
mod insurance_claim_status;
mod insurance_claim_status_group;
mod insurance_product_rule;
mod insured_status;
mod quick_compensation_state;

pub use accident_reason::AccidentReason;
pub use insurance_claim::InsuranceClaim;
pub use insurance_claim_amount::InsuranceClaimAmount;
pub use insurance_claim_event::InsuranceClaimEvent;
pub use insurance_claim_invoice::InsuranceClaimInvoice;
pub use insurance_claim_invoice_cost::InsuranceClaimInvoiceCost;
pub use insurance_claim_invoice_status::InsuranceClaimInvoiceStatus;
pub use insurance_claim_invoice_type::InsuranceClaimInvoiceType;
pub use insurance_claim_medical::InsuranceClaimMedical;
pub use insurance_claim_status::InsuranceClaimStatus;
pub use insurance_claim_status_group::InsuranceClaimStatusGroup;
pub use insurance_product_rule::InsuranceProductRule;
pub use insured_status::InsuredStatus;
pub use quick_compensation_state::QuickCompensationState;
