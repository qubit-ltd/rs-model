// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual claim invoices.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::InsuranceClaimInvoiceCost;
use crate::claim::InsuranceClaimInvoiceStatus;
use crate::claim::InsuranceClaimInvoiceType;

/// A medical invoice imported into an individual insurance claim.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InsuranceClaimInvoice {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Persisted medical-record identifier within the claim.
    #[model(opaque)]
    pub claim_medical_id: Id,

    /// Persisted attachment identifier for the source invoice image.
    #[model(opaque)]
    pub attachment_id: Id,

    /// Invoice number.
    pub number: String,

    /// Total invoice amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Medical-insurance fund payment.
    #[model(money(scale = 4))]
    pub fund_paid_amount: BigDecimal,

    /// Self-paid amount.
    #[model(money(scale = 4))]
    pub self_paid_amount: BigDecimal,

    /// Self-care amount.
    #[model(money(scale = 4))]
    pub self_care_amount: BigDecimal,

    /// Amount within medical-insurance coverage.
    #[model(money(scale = 4))]
    pub medicare_amount: BigDecimal,

    /// Optional serious-illness fund payment.
    #[model(money(scale = 4))]
    pub serious_illness_paid: Option<BigDecimal>,

    /// Optional serious-illness insurance payment.
    #[model(money(scale = 4))]
    pub serious_illness_insurance_paid: Option<BigDecimal>,

    /// Optional civil-affairs subsidy payment.
    #[model(money(scale = 4))]
    pub civil_affair_subsidy_paid: Option<BigDecimal>,

    /// Optional total personal amount.
    #[model(money(scale = 4))]
    pub self_amount: Option<BigDecimal>,

    /// Whether the invoice describes a pre-existing symptom.
    pub past_symptom: bool,

    /// Medical encounter represented by the invoice.
    pub r#type: InsuranceClaimInvoiceType,

    /// Import state.
    pub status: InsuranceClaimInvoiceStatus,

    /// Whether extracted invoice data is accurate.
    pub accuracy: bool,

    /// Explanation when extracted data is inaccurate.
    pub inaccurate_reason: String,

    /// Optional supplemental personal amount.
    #[model(money(scale = 4))]
    pub self_amount_supply: Option<BigDecimal>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,

    /// Extracted medical-charge breakdowns.
    pub costs: Vec<InsuranceClaimInvoiceCost>,
}

impl InsuranceClaimInvoice {
    /// Checks whether the total invoice amount covers all component amounts.
    ///
    /// # Returns
    ///
    /// `true` when `amount` is at least the sum of all six payment components.
    #[must_use]
    pub fn check_amount(&self) -> bool {
        let optional = |value: &Option<BigDecimal>| value.clone().unwrap_or_default();
        let components = &self.fund_paid_amount
            + &self.self_care_amount
            + &self.self_paid_amount
            + optional(&self.serious_illness_paid)
            + optional(&self.serious_illness_insurance_paid)
            + optional(&self.civil_affair_subsidy_paid);
        self.amount >= components
    }
}
