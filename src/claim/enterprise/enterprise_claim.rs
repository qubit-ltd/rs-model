// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Employer-sponsored claim cases and their imported evidence.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::AccidentReason;
use crate::claim::InsuredStatus;
use crate::claim::QuickCompensationState;
use crate::claim::enterprise::EnterpriseClaimEvent;
use crate::claim::enterprise::EnterpriseClaimStatus;
use crate::claim::enterprise::EnterpriseClaimStatusGroup;
use crate::commons::Kinship;
use crate::order::Client;
use crate::product::Product;
use crate::upload::Attachment;

/// An employer-sponsored claim case containing covered parties, supporting
/// records, calculation items, and its processing history.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaim {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Employer-sponsored product under which this benefit is assessed.
    pub product: Product,

    /// Cause of the insured event.
    pub reason: AccidentReason,

    /// Treatment outcome used by the enterprise claim workflow.
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

    /// Free-form operator notes retained with the claim case.
    pub notes: String,

    /// Quick-compensation retrieval state.
    pub quick_compensation_state: QuickCompensationState,

    /// Recorded state transitions for the enterprise claim.
    pub events: Vec<EnterpriseClaimEvent>,

    /// Submitted evidence and other supporting attachments.
    pub attachment_list: Vec<Attachment>,

    /// UTC time at which the enterprise claim was submitted.
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
            EnterpriseClaimStatus::NotSubmitted | EnterpriseClaimStatus::SystemRejected
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
            EnterpriseClaimStatus::ClaimApplicationAudited | EnterpriseClaimStatus::TemporarySaved
        )
    }
}
