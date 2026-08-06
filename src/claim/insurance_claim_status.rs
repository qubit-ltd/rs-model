// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Detailed individual claim workflow states.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

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
    #[inline(always)]
    #[must_use]
    pub const fn status_group(self) -> InsuranceClaimStatusGroup {
        status_group_for(self)
    }

    /// Returns the detailed states treated as unfinished by the Java model.
    ///
    /// # Returns
    ///
    /// A stable slice containing every unfinished claim state in source order.
    #[inline(always)]
    #[must_use]
    pub const fn list_not_finished_status() -> &'static [Self] {
        not_finished_statuses()
    }
}

/// Maps a detailed individual claim status to its source reporting group.
const fn status_group_for(
    status: InsuranceClaimStatus,
) -> InsuranceClaimStatusGroup {
    match status {
        InsuranceClaimStatus::NotSubmitted => {
            InsuranceClaimStatusGroup::NotSubmitted
        }
        InsuranceClaimStatus::ClaimApplicationWaitAudit
        | InsuranceClaimStatus::ClaimApplicationAudited
        | InsuranceClaimStatus::TemporarySaved => {
            InsuranceClaimStatusGroup::PendingCase
        }
        InsuranceClaimStatus::SystemAudited => {
            InsuranceClaimStatusGroup::Unreached
        }
        InsuranceClaimStatus::SystemRejected => {
            InsuranceClaimStatusGroup::Rejected
        }
        InsuranceClaimStatus::WaitInsuranceCompanyAudited => {
            InsuranceClaimStatusGroup::Registed
        }
        InsuranceClaimStatus::InsuranceCompanyAccepted => {
            InsuranceClaimStatusGroup::UnderReview
        }
        InsuranceClaimStatus::InsuranceCompanyRejected
        | InsuranceClaimStatus::InsuranceCompanyAnnulOrRefused => {
            InsuranceClaimStatusGroup::AuditRejection
        }
        InsuranceClaimStatus::InsuranceCompanyCompleted => {
            InsuranceClaimStatusGroup::Completed
        }
        InsuranceClaimStatus::Canceled => InsuranceClaimStatusGroup::Canceld,
    }
}

/// Returns the stable source-order list of unfinished individual claim states.
const fn not_finished_statuses() -> &'static [InsuranceClaimStatus] {
    &[
        InsuranceClaimStatus::NotSubmitted,
        InsuranceClaimStatus::ClaimApplicationWaitAudit,
        InsuranceClaimStatus::ClaimApplicationAudited,
        InsuranceClaimStatus::TemporarySaved,
        InsuranceClaimStatus::SystemRejected,
        InsuranceClaimStatus::WaitInsuranceCompanyAudited,
        InsuranceClaimStatus::InsuranceCompanyAccepted,
        InsuranceClaimStatus::InsuranceCompanyRejected,
    ]
}

#[cfg(test)]
mod tests {
    use super::{InsuranceClaimStatus, InsuranceClaimStatusGroup};

    /// Exercises the public forwarding API in the library test binary.
    #[test]
    fn public_status_apis_delegate_to_the_source_mappings() {
        let status_group: fn(
            InsuranceClaimStatus,
        ) -> InsuranceClaimStatusGroup = InsuranceClaimStatus::status_group;
        let unfinished: fn() -> &'static [InsuranceClaimStatus] =
            InsuranceClaimStatus::list_not_finished_status;

        assert_eq!(
            status_group(InsuranceClaimStatus::InsuranceCompanyCompleted),
            InsuranceClaimStatusGroup::Completed
        );
        assert_eq!(unfinished().len(), 8);
    }
}
