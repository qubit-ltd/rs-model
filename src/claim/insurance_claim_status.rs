// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Detailed individual claim workflow states.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::claim::InsuranceClaimStatusGroup;

/// Describes the detailed processing state of an individual insurance claim.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimStatus {
    /// The claim has not been submitted.
    NotSubmitted,
    /// The claim application is awaiting audit.
    ClaimApplicationWaitAudit,
    /// The claim application passed audit.
    ClaimApplicationAudited,
    /// The claim was temporarily saved for later processing.
    TemporarySaved,
    /// The system audit succeeded without sending data to the insurer.
    SystemAudited,
    /// The system rejected the claim.
    SystemRejected,
    /// The claim is awaiting insurer review.
    WaitInsuranceCompanyAudited,
    /// The insurer accepted the case for processing.
    InsuranceCompanyAccepted,
    /// The insurer rejected the claim during audit.
    InsuranceCompanyRejected,
    /// The insurer completed the claim.
    InsuranceCompanyCompleted,
    /// The insurer annulled or refused the claim.
    InsuranceCompanyAnnulOrRefused,
    /// The user cancelled the claim.
    Canceled,
}

impl InsuranceClaimStatus {
    /// Returns the high-level group corresponding to this detailed state.
    ///
    /// # Returns
    ///
    /// The source-domain reporting group for this state.
    #[must_use]
    pub const fn status_group(self) -> InsuranceClaimStatusGroup {
        match self {
            Self::NotSubmitted => InsuranceClaimStatusGroup::NotSubmitted,
            Self::ClaimApplicationWaitAudit
            | Self::ClaimApplicationAudited
            | Self::TemporarySaved => InsuranceClaimStatusGroup::PendingCase,
            Self::SystemAudited => InsuranceClaimStatusGroup::Unreached,
            Self::SystemRejected => InsuranceClaimStatusGroup::Rejected,
            Self::WaitInsuranceCompanyAudited => {
                InsuranceClaimStatusGroup::Registed
            }
            Self::InsuranceCompanyAccepted => {
                InsuranceClaimStatusGroup::UnderReview
            }
            Self::InsuranceCompanyRejected
            | Self::InsuranceCompanyAnnulOrRefused => {
                InsuranceClaimStatusGroup::AuditRejection
            }
            Self::InsuranceCompanyCompleted => {
                InsuranceClaimStatusGroup::Completed
            }
            Self::Canceled => InsuranceClaimStatusGroup::Canceld,
        }
    }

    /// Returns the detailed states treated as unfinished by the Java model.
    ///
    /// # Returns
    ///
    /// A stable slice containing every unfinished claim state in source order.
    #[must_use]
    pub const fn list_not_finished_status() -> &'static [Self] {
        &[
            Self::NotSubmitted,
            Self::ClaimApplicationWaitAudit,
            Self::ClaimApplicationAudited,
            Self::TemporarySaved,
            Self::SystemRejected,
            Self::WaitInsuranceCompanyAudited,
            Self::InsuranceCompanyAccepted,
            Self::InsuranceCompanyRejected,
        ]
    }
}
