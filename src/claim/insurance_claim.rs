// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual insurance claims.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

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

/// An individual claim with insured event, payment, documents, and workflow
/// data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct InsuranceClaim {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Insurance product being claimed.
    pub product: Product,

    /// Insurance company information.
    #[model(opaque)]
    pub company: Info,

    /// Claim source information.
    #[model(opaque)]
    pub source: Info,

    /// Cause of the insured event.
    pub reason: AccidentReason,

    /// Insurance policy number.
    #[redact(level = "secret")]
    pub policy_number: String,

    /// Insured person.
    pub insured: Client,

    /// Optional insured-person address.
    pub insured_address: Option<Address>,

    /// Optional insured-person treatment outcome.
    pub insured_status: Option<InsuredStatus>,

    /// Claimant's relationship to the insured person.
    pub claimant_relation: Kinship,

    /// Claimant information.
    pub claimant: Client,

    /// Optional claimant address.
    pub claimant_address: Option<Address>,

    /// Date of the insured event.
    pub accident_date: NaiveDate,

    /// Place of the insured event.
    pub accident_place: String,

    /// Description of the insured event.
    pub accident_description: String,

    /// Optional treating hospital.
    #[model(opaque)]
    pub hospital: Option<Info>,

    /// Optional treatment start date.
    pub treatment_start_date: Option<NaiveDate>,

    /// Optional treatment end date.
    pub treatment_end_date: Option<NaiveDate>,

    /// Quick-compensation retrieval state.
    pub quick_compensation_state: QuickCompensationState,

    /// Optional claim currency.
    pub currency: Option<Currency>,

    /// Optional total amount already paid by the claimant.
    #[model(money(scale = 4))]
    pub total_paid_amount: Option<BigDecimal>,

    /// Claim payment payee name.
    #[redact(level = "secret")]
    pub payee_name: String,

    /// Claim payment account.
    pub account: Account,

    /// Claim number.
    #[redact(level = "secret")]
    pub number: String,

    /// Optional UTC issue timestamp.
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

    /// Optional source-order key-value payload.
    #[model(opaque)]
    pub payload: Option<Vec<(String, String)>>,

    /// Supporting attachments.
    pub attachment_list: Vec<Attachment>,

    /// Workflow events.
    pub events: Vec<InsuranceClaimEvent>,

    /// Claimed medical encounters.
    pub medical_list: Vec<InsuranceClaimMedical>,

    /// Invoices saved for the claim.
    pub saved_invoices: Vec<InsuranceClaimInvoice>,

    /// Claim amount summary.
    pub amount: InsuranceClaimAmount,

    /// UTC creation timestamp.
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
            InsuranceClaimStatus::ClaimApplicationAudited
                | InsuranceClaimStatus::TemporarySaved
        )
    }
}
