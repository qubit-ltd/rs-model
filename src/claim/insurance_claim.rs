// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual claim cases submitted under a personal insurance policy.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::claim::AccidentReason;
use crate::claim::InsuranceClaimAmount;
use crate::claim::InsuranceClaimEvent;
use crate::claim::InsuranceClaimInvoice;
use crate::claim::InsuranceClaimMedical;
use crate::claim::InsuranceClaimStatus;
use crate::claim::InsuranceClaimStatusGroup;
use crate::claim::InsuredStatus;
use crate::claim::QuickCompensationState;
use crate::commons::Currency;
use crate::commons::Kinship;
use crate::contact::Address;
use crate::order::Client;
use crate::payment::Account;
use crate::product::Product;
use crate::upload::Attachment;

/// A personal claim case that joins the covered event, submitted evidence,
/// payment instructions, medical evidence, and processing history.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct InsuranceClaim {
    /// Globally unique claim-case identifier, assigned when the report is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Insurance product being claimed.
    pub product: Product,

    /// Insurance company information.
    #[model(opaque)]
    pub company: Info,

    /// Channel or upstream system from which the claim report originated.
    #[model(opaque)]
    pub source: Info,

    /// Cause of the insured event.
    pub reason: AccidentReason,

    /// Insurance policy number.
    #[redact(level = "secret")]
    pub policy_number: String,

    /// Insured person.
    pub insured: Client,

    /// Insured person's address when it was supplied with the claim; `None`
    /// means the report did not include one.
    pub insured_address: Option<Address>,

    /// Reported outcome of the insured person's treatment; absent until known.
    pub insured_status: Option<InsuredStatus>,

    /// Claimant's relationship to the insured person.
    pub claimant_relation: Kinship,

    /// Claimant information.
    pub claimant: Client,

    /// Claimant's address when it differs from or is supplied separately from
    /// the insured person's address.
    pub claimant_address: Option<Address>,

    /// Date of the insured event.
    pub accident_date: NaiveDate,

    /// Place of the insured event.
    pub accident_place: String,

    /// Description of the insured event.
    pub accident_description: String,

    /// Treating hospital when the reported event involved medical care; `None`
    /// covers non-medical claims or unavailable hospital data.
    #[model(opaque)]
    pub hospital: Option<Info>,

    /// Start of the latest treatment, visit, or admission period, if reported.
    pub treatment_start_date: Option<NaiveDate>,

    /// End of the latest treatment, visit, or admission period, if known.
    pub treatment_end_date: Option<NaiveDate>,

    /// Quick-compensation retrieval state.
    pub quick_compensation_state: QuickCompensationState,

    /// Currency of the reported invoice amounts; absent when the source does
    /// not identify one.
    pub currency: Option<Currency>,

    /// Total amount reported with the claim, before the insurer calculates a
    /// payable benefit.
    #[model(money(scale = 4))]
    pub total_paid_amount: Option<BigDecimal>,

    /// Claim payment payee name.
    #[redact(level = "secret")]
    pub payee_name: String,

    /// Claim payment account.
    pub account: Account,

    /// Business registration number used to track this claim externally.
    #[redact(level = "secret")]
    pub number: String,

    /// UTC time at which the claimant initiated the report.
    #[model(time(precision = second, normalization = utc))]
    pub issue_time: Option<DateTime<Utc>>,

    /// Optional UTC cancellation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// Optional UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// Detailed claim workflow state.
    pub status: InsuranceClaimStatus,

    /// High-level claim workflow group.
    pub status_group: InsuranceClaimStatusGroup,

    /// Claim notes.
    pub notes: String,

    /// Additional source-order attributes; absent when no upstream payload was
    /// supplied.
    #[model(sequence(min_items = 1, max_items = 8), opaque)]
    pub payload: Option<Vec<(String, String)>>,

    /// Supporting attachments.
    pub attachment_list: Vec<Attachment>,

    /// Claim events, with the newest transition conventionally supplied first.
    pub events: Vec<InsuranceClaimEvent>,

    /// Claimed medical encounters.
    pub medical_list: Vec<InsuranceClaimMedical>,

    /// Invoices saved for the claim.
    pub saved_invoices: Vec<InsuranceClaimInvoice>,

    /// Claim amount summary.
    pub amount: InsuranceClaimAmount,

    /// UTC submission time recorded when the claim case is created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl InsuranceClaim {
    /// Returns whether the current state permits claimant-side operations.
    ///
    /// # Returns
    ///
    /// `true` for an unsubmitted claim or a claim rejected by the system or
    /// insurer.
    #[must_use]
    pub const fn allow_client_operation(&self) -> bool {
        matches!(
            self.status,
            InsuranceClaimStatus::NotSubmitted
                | InsuranceClaimStatus::SystemRejected
                | InsuranceClaimStatus::InsuranceCompanyRejected
        )
    }

    /// Returns whether the current state permits a system rejection.
    ///
    /// # Returns
    ///
    /// `true` while the claim application is awaiting, passed, or temporarily
    /// saved for audit.
    #[must_use]
    pub const fn allow_system_reject(&self) -> bool {
        matches!(
            self.status,
            InsuranceClaimStatus::ClaimApplicationWaitAudit
                | InsuranceClaimStatus::ClaimApplicationAudited
                | InsuranceClaimStatus::TemporarySaved
        )
    }

    /// Returns whether the current state permits system acceptance.
    ///
    /// # Returns
    ///
    /// `true` after application audit or while the claim is temporarily saved.
    #[must_use]
    pub const fn allow_system_accept(&self) -> bool {
        matches!(
            self.status,
            InsuranceClaimStatus::ClaimApplicationAudited | InsuranceClaimStatus::TemporarySaved
        )
    }
}
