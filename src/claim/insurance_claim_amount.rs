// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Monetary totals accumulated for an individual claim.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Aggregates billed, covered, claimed, and ultimately paid amounts for one
/// claim case.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InsuranceClaimAmount {
    /// Typed identifier used when this claim-amount snapshot is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Total medical expense across the reported encounters, if settlement data
    /// has been received.
    #[model(money(scale = 4))]
    pub total_amount: Option<BigDecimal>,

    /// Expense classified within medical-insurance coverage, when settlement
    /// data supplies that allocation.
    #[model(money(scale = 4))]
    pub medicare_amount: Option<BigDecimal>,

    /// Expense the patient paid outside coverage, when reported by settlement
    /// data.
    #[model(money(scale = 4))]
    pub self_paid_amount: Option<BigDecimal>,

    /// Expense the patient must bear for partially covered items, when known.
    #[model(money(scale = 4))]
    pub self_care_amount: Option<BigDecimal>,

    /// Payment from the medical-insurance fund, when supplied by settlement
    /// data.
    #[model(money(scale = 4))]
    pub fund_paid_amount: Option<BigDecimal>,

    /// Serious-illness assistance payment, when the applicable program paid.
    #[model(money(scale = 4))]
    pub serious_illness_paid: Option<BigDecimal>,

    /// Serious-illness insurance payment, when the applicable program paid.
    #[model(money(scale = 4))]
    pub serious_illness_insurance_paid: Option<BigDecimal>,

    /// Civil-affairs subsidy payment, when a subsidy was applied.
    #[model(money(scale = 4))]
    pub civil_affair_subsidy_paid: Option<BigDecimal>,

    /// Portion of the claimant's self-paid expense requested for reimbursement.
    #[model(money(scale = 4))]
    pub self_paid_claim_amount: Option<BigDecimal>,

    /// Portion of the claimant's self-care expense requested for reimbursement.
    #[model(money(scale = 4))]
    pub self_care_claim_amount: Option<BigDecimal>,

    /// Aggregate amount requested from the insurer, once the claim split is
    /// calculated.
    #[model(money(scale = 4))]
    pub total_claim_amount: Option<BigDecimal>,

    /// Final reimbursement of the self-paid portion, once payment is settled.
    #[model(money(scale = 4))]
    pub actual_self_paid_amount: Option<BigDecimal>,

    /// Final reimbursement of the self-care portion, once payment is settled.
    #[model(money(scale = 4))]
    pub actual_self_care_amount: Option<BigDecimal>,

    /// Final benefit amount actually paid by the insurer.
    #[model(money(scale = 4))]
    pub actual_paid_amount: Option<BigDecimal>,

    /// Whether the final payment figures were manually or systemically
    /// reconciled after calculation.
    pub paid_amount_calibration: bool,

    /// Date on which the insurer paid the benefit; absent before payment.
    pub pay_time: Option<NaiveDate>,

    /// Date on which the claim case was closed; absent while it remains open.
    pub endcase_date: Option<NaiveDate>,

    /// Opaque upstream payload retained with the calculated amounts.
    pub payload: Option<String>,

    /// UTC submission time for this claim-amount snapshot.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
