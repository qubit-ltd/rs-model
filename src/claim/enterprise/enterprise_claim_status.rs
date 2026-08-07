// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Detailed enterprise claim workflow states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::enterprise::EnterpriseClaimStatusGroup;

/// Describes the detailed processing state of an enterprise insurance claim.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseClaimStatus {
    /// The claim has not been submitted.
    NotSubmitted,
    /// The claim application is awaiting audit.
    ClaimApplicationWaitAudit,
    /// The system rejected the claim.
    SystemRejected,
    /// The claim application passed audit.
    ClaimApplicationAudited,
    /// The claim was temporarily saved for later processing.
    TemporarySaved,
    /// The claim is awaiting insurer review.
    WaitInsuranceCompanyAudited,
    /// The insurer completed the claim.
    InsuranceCompanyCompleted,
    /// The user cancelled the claim.
    Canceled,
}

impl EnterpriseClaimStatus {
    /// Returns the high-level group corresponding to this detailed state.
    ///
    /// # Returns
    ///
    /// The source-domain reporting group for this state.
    #[must_use]
    pub const fn status_group(self) -> EnterpriseClaimStatusGroup {
        match self {
            Self::NotSubmitted => EnterpriseClaimStatusGroup::NotSubmitted,
            Self::ClaimApplicationWaitAudit => {
                EnterpriseClaimStatusGroup::Register
            }
            Self::SystemRejected => EnterpriseClaimStatusGroup::Reject,
            Self::ClaimApplicationAudited
            | Self::TemporarySaved
            | Self::WaitInsuranceCompanyAudited => {
                EnterpriseClaimStatusGroup::Audit
            }
            Self::InsuranceCompanyCompleted => {
                EnterpriseClaimStatusGroup::Complete
            }
            Self::Canceled => EnterpriseClaimStatusGroup::Cancel,
        }
    }

    /// Returns the detailed states treated as unfinished by the Java model.
    ///
    /// # Returns
    ///
    /// A stable slice containing every unfinished enterprise claim state.
    #[must_use]
    pub const fn list_not_finished_status() -> &'static [Self] {
        &[
            Self::NotSubmitted,
            Self::ClaimApplicationWaitAudit,
            Self::SystemRejected,
            Self::ClaimApplicationAudited,
            Self::TemporarySaved,
            Self::WaitInsuranceCompanyAudited,
        ]
    }
}
