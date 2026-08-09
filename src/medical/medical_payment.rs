// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment components produced by a medical-insurance settlement.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Allocates a settlement across insurance funds, personal payment channels,
/// and public subsidies. Values preserve the upstream settlement allocation and
/// are not inferred from one another.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalPayment {
    /// Patient self-care amount for partially covered items.
    #[model(money(scale = 4))]
    pub self_care: BigDecimal,

    /// Patient self-paid amount for uncovered items.
    #[model(money(scale = 4))]
    pub self_paid: BigDecimal,

    /// Total expense within medical-insurance coverage.
    #[model(money(scale = 4))]
    pub within_medicare: BigDecimal,

    /// Total amount paid by medical-insurance funds.
    #[model(money(scale = 4))]
    pub medicare_paid: BigDecimal,

    /// Medical-insurance deductible.
    #[model(money(scale = 4))]
    pub medicare_deductible: BigDecimal,

    /// Patient liability that remains after allocation of covered expenses.
    #[model(money(scale = 4))]
    pub personal_burden: BigDecimal,

    /// Advance payment collected when the patient is transferred or referred.
    #[model(money(scale = 4))]
    pub transfer_prepaid: BigDecimal,

    /// Patient payment determined by the applicable reimbursement tier.
    #[model(money(scale = 4))]
    pub sectional_paid: BigDecimal,

    /// Amount contractually borne by the treating hospital.
    #[model(money(scale = 4))]
    pub hospital_burden: BigDecimal,

    /// Pooled-fund payment; its underlying program depends on the patient's
    /// insured identity.
    #[model(money(scale = 4))]
    pub pool_fund_paid: BigDecimal,

    /// Amount paid by the civil-servant fund.
    #[model(money(scale = 4))]
    pub civil_servant_fund_paid: BigDecimal,

    /// Amount paid by serious-disease assistance.
    #[model(money(scale = 4))]
    pub serious_disease_assistance_paid: BigDecimal,

    /// Amount paid by serious-disease insurance.
    #[model(money(scale = 4))]
    pub serious_disease_insurance_paid: BigDecimal,

    /// Amount paid by civil-affairs assistance.
    #[model(money(scale = 4))]
    pub civil_affairs_assistance_paid: BigDecimal,

    /// Amount paid by other funds.
    #[model(money(scale = 4))]
    pub other_fund_paid: BigDecimal,

    /// Amount debited from the patient's personal medical-insurance account.
    #[model(money(scale = 4))]
    pub account_paid: BigDecimal,

    /// Remaining patient-account balance.
    #[model(money(scale = 4))]
    pub account_balance: BigDecimal,

    /// Cash prepaid amount.
    #[model(money(scale = 4))]
    pub cash_prepaid: BigDecimal,

    /// Additional cash payment.
    #[model(money(scale = 4))]
    pub cash_recharge: BigDecimal,

    /// Cash refund amount.
    #[model(money(scale = 4))]
    pub cash_refund: BigDecimal,

    /// Cheque prepaid amount.
    #[model(money(scale = 4))]
    pub cheque_prepaid: BigDecimal,

    /// Additional cheque payment.
    #[model(money(scale = 4))]
    pub cheque_recharge: BigDecimal,

    /// Cheque refund amount.
    #[model(money(scale = 4))]
    pub cheque_refund: BigDecimal,

    /// Bank-transfer prepaid amount.
    #[model(money(scale = 4))]
    pub bank_transfer_prepaid: BigDecimal,

    /// Additional bank-transfer payment.
    #[model(money(scale = 4))]
    pub bank_transfer_recharge: BigDecimal,

    /// Bank-transfer refund amount.
    #[model(money(scale = 4))]
    pub bank_transfer_refund: BigDecimal,
}
