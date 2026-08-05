// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual and enterprise insurance-claim models.

mod accident_reason;
pub mod enterprise;
mod insurance_claim_invoice_status;
mod insurance_claim_invoice_type;
mod insurance_claim_status;
mod insurance_claim_status_group;
mod insured_status;
mod quick_compensation_state;

pub use accident_reason::AccidentReason;
pub use insurance_claim_invoice_status::InsuranceClaimInvoiceStatus;
pub use insurance_claim_invoice_type::InsuranceClaimInvoiceType;
pub use insurance_claim_status::InsuranceClaimStatus;
pub use insurance_claim_status_group::InsuranceClaimStatusGroup;
pub use insured_status::InsuredStatus;
pub use quick_compensation_state::QuickCompensationState;
