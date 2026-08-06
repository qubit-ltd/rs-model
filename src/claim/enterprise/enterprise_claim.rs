// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise insurance claims.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    claim::enterprise::{
        EnterpriseClaimEvent,
        EnterpriseClaimStatus,
        EnterpriseClaimStatusGroup,
    },
    claim::{
        AccidentReason,
        InsuredStatus,
        QuickCompensationState,
    },
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
    #[must_use]
    pub const fn allow_client_operation(&self) -> bool {
        matches!(
            self.status,
            EnterpriseClaimStatus::NotSubmitted
                | EnterpriseClaimStatus::SystemRejected
        )
    }

    /// Returns whether the current state permits rejection back to the
    /// claimant.
    ///
    /// # Returns
    ///
    /// `true` while the application awaits audit, passed audit, or is
    /// temporarily saved.
    #[must_use]
    pub const fn allow_reject(&self) -> bool {
        matches!(
            self.status,
            EnterpriseClaimStatus::ClaimApplicationWaitAudit
                | EnterpriseClaimStatus::ClaimApplicationAudited
                | EnterpriseClaimStatus::TemporarySaved
        )
    }

    /// Returns whether the current state permits administrator operations.
    ///
    /// # Returns
    ///
    /// `true` after application audit or while the claim is temporarily saved.
    #[must_use]
    pub const fn allow_admin_operation(&self) -> bool {
        matches!(
            self.status,
            EnterpriseClaimStatus::ClaimApplicationAudited
                | EnterpriseClaimStatus::TemporarySaved
        )
    }
}
