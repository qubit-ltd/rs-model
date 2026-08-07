// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim invoices.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::claim::enterprise::EnterpriseClaimSelfCareItem;
use crate::claim::enterprise::SaveStatus;

/// A medical invoice imported into an enterprise insurance claim.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct EnterpriseClaimInvoice {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted claim identifier.
    pub claim_id: i64,

    /// Persisted medical-record identifier within the claim.
    pub claim_medical_id: i64,

    /// Persisted attachment identifier for the source invoice image.
    pub attachment_id: i64,

    /// Invoice number.
    pub number: String,

    /// Deductible applied to the invoice.
    #[model(money(scale = 4))]
    pub deductible: BigDecimal,

    /// Total invoice amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Self-paid amount.
    #[model(money(scale = 4))]
    pub self_paid_amount: BigDecimal,

    /// Self-care amount.
    #[model(money(scale = 4))]
    pub self_care_amount: BigDecimal,

    /// Pooled-fund payment.
    #[model(money(scale = 4))]
    pub fund_paid_amount: BigDecimal,

    /// Serious-illness assistance amount.
    #[model(money(scale = 4))]
    pub serious_illness_amount: BigDecimal,

    /// Serious-illness insurance amount.
    #[model(money(scale = 4))]
    pub serious_illness_insurance_amount: BigDecimal,

    /// Non-reimbursable amount.
    #[model(money(scale = 4))]
    pub no_reimbursement_amount: BigDecimal,

    /// Invalid charge amount.
    #[model(money(scale = 4))]
    pub invalid_amount: BigDecimal,

    /// Class-B self-care amount.
    #[model(money(scale = 4))]
    pub class_b_self_care_amount: BigDecimal,

    /// Total personal amount.
    #[model(money(scale = 4))]
    pub self_amount: BigDecimal,

    /// Civil-affairs subsidy amount.
    #[model(money(scale = 4))]
    pub civil_affair_subsidy_amount: BigDecimal,

    /// Amount within medical-insurance coverage.
    #[model(money(scale = 4))]
    pub medicare_amount: BigDecimal,

    /// Invoice source information.
    #[model(opaque)]
    pub source: InfoWithEntity,

    /// Optional operator name.
    pub operator_name: Option<String>,

    /// Import state.
    pub status: SaveStatus,

    /// Whether extracted invoice data is accurate.
    pub accuracy: bool,

    /// Explanation when extracted data is inaccurate.
    pub inaccurate_reason: String,

    /// Class-B self-care details.
    pub self_care_items: Vec<EnterpriseClaimSelfCareItem>,

    /// Claim calculation base.
    #[model(money(scale = 4))]
    pub claim_base: BigDecimal,

    /// Calculated claim amount.
    #[model(money(scale = 4))]
    pub claim_amount: BigDecimal,

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

impl EnterpriseClaimInvoice {
    /// Checks whether the invoice total covers all payment components.
    ///
    /// # Returns
    ///
    /// `true` when `amount` is at least the source-domain component sum.
    #[must_use]
    pub fn check_amount(&self) -> bool {
        let components = &self.fund_paid_amount
            + &self.serious_illness_amount
            + &self.serious_illness_insurance_amount
            + &self.civil_affair_subsidy_amount
            + &self.self_care_amount
            + &self.self_paid_amount;
        self.amount >= components
    }

    /// Checks whether every Class-B self-care ratio lies in the inclusive unit
    /// interval.
    ///
    /// # Returns
    ///
    /// `true` for an empty list or when every ratio is between zero and one.
    #[must_use]
    pub fn check_self_care_items(&self) -> bool {
        self.self_care_items
            .iter()
            .all(|item| (0.0..=1.0).contains(&item.ratio))
    }
}
