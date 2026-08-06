// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Detailed enterprise claim workflow states.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

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
    #[inline(always)]
    #[must_use]
    pub const fn status_group(self) -> EnterpriseClaimStatusGroup {
        status_group_for(self)
    }

    /// Returns the detailed states treated as unfinished by the Java model.
    ///
    /// # Returns
    ///
    /// A stable slice containing every unfinished enterprise claim state.
    #[inline(always)]
    #[must_use]
    pub const fn list_not_finished_status() -> &'static [Self] {
        not_finished_statuses()
    }
}

/// Maps a detailed enterprise claim status to its source reporting group.
const fn status_group_for(
    status: EnterpriseClaimStatus,
) -> EnterpriseClaimStatusGroup {
    match status {
        EnterpriseClaimStatus::NotSubmitted => {
            EnterpriseClaimStatusGroup::NotSubmitted
        }
        EnterpriseClaimStatus::ClaimApplicationWaitAudit => {
            EnterpriseClaimStatusGroup::Register
        }
        EnterpriseClaimStatus::SystemRejected => {
            EnterpriseClaimStatusGroup::Reject
        }
        EnterpriseClaimStatus::ClaimApplicationAudited
        | EnterpriseClaimStatus::TemporarySaved
        | EnterpriseClaimStatus::WaitInsuranceCompanyAudited => {
            EnterpriseClaimStatusGroup::Audit
        }
        EnterpriseClaimStatus::InsuranceCompanyCompleted => {
            EnterpriseClaimStatusGroup::Complete
        }
        EnterpriseClaimStatus::Canceled => EnterpriseClaimStatusGroup::Cancel,
    }
}

/// Returns the stable source-order list of unfinished enterprise claim states.
const fn not_finished_statuses() -> &'static [EnterpriseClaimStatus] {
    &[
        EnterpriseClaimStatus::NotSubmitted,
        EnterpriseClaimStatus::ClaimApplicationWaitAudit,
        EnterpriseClaimStatus::SystemRejected,
        EnterpriseClaimStatus::ClaimApplicationAudited,
        EnterpriseClaimStatus::TemporarySaved,
        EnterpriseClaimStatus::WaitInsuranceCompanyAudited,
    ]
}

#[cfg(test)]
mod tests {
    use super::{EnterpriseClaimStatus, EnterpriseClaimStatusGroup};

    /// Exercises the public forwarding API in the library test binary.
    #[test]
    fn public_status_apis_delegate_to_the_source_mappings() {
        let status_group: fn(
            EnterpriseClaimStatus,
        ) -> EnterpriseClaimStatusGroup = EnterpriseClaimStatus::status_group;
        let unfinished: fn() -> &'static [EnterpriseClaimStatus] =
            EnterpriseClaimStatus::list_not_finished_status;

        assert_eq!(
            status_group(EnterpriseClaimStatus::InsuranceCompanyCompleted),
            EnterpriseClaimStatusGroup::Complete
        );
        assert_eq!(unfinished().len(), 6);
    }
}
