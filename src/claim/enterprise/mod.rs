// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise insurance-claim models and classifications.

#[allow(clippy::module_inception)]
mod enterprise_claim;
mod enterprise_claim_event;
mod enterprise_claim_invoice;
mod enterprise_claim_item;
mod enterprise_claim_item_medical;
mod enterprise_claim_item_status;
mod enterprise_claim_medical;
mod enterprise_claim_self_care_item;
mod enterprise_claim_status;
mod enterprise_claim_status_group;
mod enterprise_history_claim_amount;
mod enterprise_insured_info;
mod enterprise_insured_type;
mod enterprise_ownership;
mod history_claim_amount;
mod save_status;

pub use enterprise_claim::EnterpriseClaim;
pub use enterprise_claim_event::EnterpriseClaimEvent;
pub use enterprise_claim_invoice::EnterpriseClaimInvoice;
pub use enterprise_claim_item::EnterpriseClaimItem;
pub use enterprise_claim_item_medical::EnterpriseClaimItemMedical;
pub use enterprise_claim_item_status::EnterpriseClaimItemStatus;
pub use enterprise_claim_medical::EnterpriseClaimMedical;
pub use enterprise_claim_self_care_item::EnterpriseClaimSelfCareItem;
pub use enterprise_claim_status::EnterpriseClaimStatus;
pub use enterprise_claim_status_group::EnterpriseClaimStatusGroup;
pub use enterprise_history_claim_amount::EnterpriseHistoryClaimAmount;
pub use enterprise_insured_info::EnterpriseInsuredInfo;
pub use enterprise_insured_type::EnterpriseInsuredType;
pub use enterprise_ownership::EnterpriseOwnership;
pub use history_claim_amount::HistoryClaimAmount;
pub use save_status::SaveStatus;
