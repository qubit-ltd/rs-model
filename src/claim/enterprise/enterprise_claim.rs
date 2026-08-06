// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise insurance claims.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    claim::enterprise::{
        EnterpriseClaimEvent, EnterpriseClaimStatus, EnterpriseClaimStatusGroup,
    },
    claim::{AccidentReason, InsuredStatus, QuickCompensationState},
    commons::Kinship,
    order::Client,
    product::Product,
    upload::Attachment,
};

/// An enterprise claim with insured parties, workflow events, and attachments.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct EnterpriseClaim {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Insurance product being claimed.
    pub product: Product,
    /// Cause of the insured event.
    pub reason: AccidentReason,
    /// Insured-person treatment outcome.
    pub insured_status: InsuredStatus,
    /// Insured person.
    pub insured: Client,
    /// Claimant's relationship to the insured person.
    pub claimant_relation: Kinship,
    /// Claimant information.
    pub claimant: Client,
    /// Optional UTC issue timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub issue_time: Option<DateTime<Utc>>,
    /// Optional UTC cancellation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,
    /// Optional UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,
    /// Detailed enterprise claim workflow state.
    pub status: EnterpriseClaimStatus,
    /// High-level enterprise claim workflow group.
    pub status_group: EnterpriseClaimStatusGroup,
    /// Claim notes.
    pub notes: String,
    /// Quick-compensation retrieval state.
    pub quick_compensation_state: QuickCompensationState,
    /// Workflow events.
    pub events: Vec<EnterpriseClaimEvent>,
    /// Supporting attachments.
    pub attachment_list: Vec<Attachment>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}

impl EnterpriseClaim {
    /// Returns whether the current state permits claimant-side operations.
    ///
    /// # Returns
    ///
    /// `true` for an unsubmitted or system-rejected claim.
    #[inline(always)]
    #[must_use]
    pub const fn allow_client_operation(&self) -> bool {
        allows_client_operation(self.status)
    }

    /// Returns whether the current state permits rejection back to the
    /// claimant.
    ///
    /// # Returns
    ///
    /// `true` while the application awaits audit, passed audit, or is
    /// temporarily saved.
    #[inline(always)]
    #[must_use]
    pub const fn allow_reject(&self) -> bool {
        allows_rejection(self.status)
    }

    /// Returns whether the current state permits administrator operations.
    ///
    /// # Returns
    ///
    /// `true` after application audit or while the claim is temporarily saved.
    #[inline(always)]
    #[must_use]
    pub const fn allow_admin_operation(&self) -> bool {
        allows_admin_operation(self.status)
    }
}

/// Returns whether an enterprise claim status permits claimant-side work.
const fn allows_client_operation(status: EnterpriseClaimStatus) -> bool {
    matches!(
        status,
        EnterpriseClaimStatus::NotSubmitted
            | EnterpriseClaimStatus::SystemRejected
    )
}

/// Returns whether an enterprise claim status permits rejection to a claimant.
const fn allows_rejection(status: EnterpriseClaimStatus) -> bool {
    matches!(
        status,
        EnterpriseClaimStatus::ClaimApplicationWaitAudit
            | EnterpriseClaimStatus::ClaimApplicationAudited
            | EnterpriseClaimStatus::TemporarySaved
    )
}

/// Returns whether an enterprise claim status permits administrator work.
const fn allows_admin_operation(status: EnterpriseClaimStatus) -> bool {
    matches!(
        status,
        EnterpriseClaimStatus::ClaimApplicationAudited
            | EnterpriseClaimStatus::TemporarySaved
    )
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use qubit_mixin::Info;

    use super::{
        EnterpriseClaim, EnterpriseClaimStatus, EnterpriseClaimStatusGroup,
    };
    use crate::{
        claim::{AccidentReason, InsuredStatus, QuickCompensationState},
        commons::{Currency, Kinship, State},
        mixin::StatefulInfo,
        order::Client,
        product::{Product, Quality},
    };

    /// Builds the client required by an enterprise claim test fixture.
    fn client() -> Client {
        Client {
            id: None,
            name: "Claimant".into(),
            credential: None,
            gender: None,
            birthday: None,
            mobile: None,
            email: None,
            has_medicare: None,
            medicare_type: None,
            medicare_card: None,
            medicare_city: None,
            has_social_security: None,
            social_security_card: None,
            social_security_city: None,
            guardian: None,
            return_status: None,
            kinship: None,
            payload: None,
        }
    }

    /// Builds the product required by an enterprise claim test fixture.
    fn product() -> Product {
        let now = Utc::now();
        Product {
            id: None,
            code: "CLAIM_PRODUCT".into(),
            name: "Claim product".into(),
            app: StatefulInfo::default(),
            category: None,
            quality: Quality::BrandNew,
            currency: Currency::Cny,
            image: None,
            description: None,
            valid_from: None,
            valid_until: None,
            brand: None,
            origin: None,
            manufacturer: None,
            seller: Info::default(),
            sale_from: now.naive_utc(),
            sale_until: None,
            need_delivery: false,
            allow_return: false,
            allow_change: false,
            need_client: false,
            constraint: None,
            items: Vec::new(),
            state: State::Normal,
            create_time: now,
            modify_time: None,
            delete_time: None,
        }
    }

    /// Builds the minimum complete enterprise claim for a selected status.
    fn claim(status: EnterpriseClaimStatus) -> EnterpriseClaim {
        EnterpriseClaim {
            id: None,
            product: product(),
            reason: AccidentReason::Disease,
            insured_status: InsuredStatus::Recovery,
            insured: client(),
            claimant_relation: Kinship::Self_,
            claimant: client(),
            issue_time: None,
            cancel_time: None,
            complete_time: None,
            status,
            status_group: EnterpriseClaimStatusGroup::NotSubmitted,
            notes: String::new(),
            quick_compensation_state: QuickCompensationState::Success,
            events: Vec::new(),
            attachment_list: Vec::new(),
            create_time: Utc::now(),
            modify_time: None,
        }
    }

    /// Executes the public claimant-operation forwarding API in the library.
    #[test]
    fn public_client_operation_api_delegates_to_the_source_mapping() {
        let allow_client: fn(&EnterpriseClaim) -> bool =
            EnterpriseClaim::allow_client_operation;

        assert!(allow_client(&claim(EnterpriseClaimStatus::NotSubmitted)));
        assert!(!allow_client(&claim(
            EnterpriseClaimStatus::ClaimApplicationWaitAudit
        )));
    }
}
