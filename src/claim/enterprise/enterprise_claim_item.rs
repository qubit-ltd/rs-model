// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Per-category allocation and reimbursement calculations for enterprise claims.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::claim::enterprise::EnterpriseClaimItemStatus;
use crate::claim::enterprise::EnterpriseClaimMedical;
use crate::claim::enterprise::EnterpriseHistoryClaimAmount;
use crate::claim::enterprise::EnterpriseInsuredType;
use crate::commons::DictEntryInfo;

/// A calculated portion of an enterprise claim for one medical category and
/// covered-person type.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaimItem {
    /// Typed identifier used when this enterprise claim allocation is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Medical-category dictionary entry.
    pub medical_category: DictEntryInfo,

    /// Insured-person classification used by this allocation, absent when the
    /// source does not distinguish it.
    pub insured_type: Option<EnterpriseInsuredType>,

    /// Total submitted amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Combined pooled-fund expenditure, including serious-illness assistance.
    #[model(money(scale = 4))]
    pub overall_fund_amount: BigDecimal,

    /// Charges excluded because they are invalid for reimbursement.
    #[model(money(scale = 4))]
    pub invalid_amount: BigDecimal,

    /// Deductible.
    #[model(money(scale = 4))]
    pub deductible: BigDecimal,

    /// Patient-borne expense after medical-insurance allocation.
    #[model(money(scale = 4))]
    pub self_amount: BigDecimal,

    /// Eligible base on which the enterprise benefit is calculated.
    #[model(money(scale = 4))]
    pub claim_base: BigDecimal,

    /// Calculated claim amount.
    #[model(money(scale = 4))]
    pub claim_amount: BigDecimal,

    /// Actual approved claim amount.
    #[model(money(scale = 4))]
    pub actual_claim_amount: BigDecimal,

    /// Eligible amount exceeding the product's reimbursement ceiling.
    #[model(money(scale = 4))]
    pub over_upper_limit: BigDecimal,

    /// Serious-illness assistance amount.
    #[model(money(scale = 4))]
    pub serious_illness_amount: BigDecimal,

    /// Serious-illness insurance amount.
    #[model(money(scale = 4))]
    pub serious_illness_insurance_amount: BigDecimal,

    /// Supplemental benefit supplied by the Yangtze ownership program.
    #[model(money(scale = 4))]
    pub yangzi_supply: BigDecimal,

    /// Hospital summary derived from attached medicals; absent when none exist.
    pub hospital_name: Option<String>,

    /// Hospital grade derived from attached medicals; absent when none is known.
    pub hospital_level: Option<i32>,

    /// Disease code from the first recorded medical; absent when it has no
    /// disease.
    pub disease_code: Option<String>,

    /// Actual transferred amount.
    #[model(money(scale = 4))]
    pub actual_paid_amount: BigDecimal,

    /// Benefit payment date, absent until the enterprise claim is paid.
    pub paid_date: Option<NaiveDate>,

    /// Enterprise case-closing date, absent while the allocation remains open.
    pub endcase_date: Option<NaiveDate>,

    /// Operator associated with the payment outcome, if supplied by the source.
    pub operator_name: Option<String>,

    /// Source description of the payment outcome, if the source provided one.
    pub description: Option<String>,

    /// Calculation state.
    pub status: EnterpriseClaimItemStatus,

    /// Medical encounters included in this calculation.
    pub medicals: Vec<EnterpriseClaimMedical>,

    /// Historical claim amounts used by the calculation.
    pub history_claim_amount: EnterpriseHistoryClaimAmount,

    /// Whether this calculation has already applied its deductible, preventing
    /// it from being deducted again.
    pub deduct_deductible: bool,

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

impl EnterpriseClaimItem {
    /// Refreshes denormalized hospital and disease summaries from the attached
    /// medical encounters.
    ///
    /// An empty list leaves existing summaries unchanged. A single hospital
    /// keeps its name and level; multiple hospitals use the source label `其他`
    /// and the greatest available level. The first recorded medical supplies
    /// the disease code, including clearing it when that record has no disease.
    pub fn init_hospital_and_disease(&mut self) {
        let Some(first) = self.medicals.first() else {
            return;
        };
        let first_hospital_name = first
            .hospital
            .as_ref()
            .map(|hospital| hospital.name.clone());
        let all_same_hospital = self.medicals.iter().all(|medical| {
            medical.hospital.as_ref().map(|hospital| &hospital.name)
                == first.hospital.as_ref().map(|hospital| &hospital.name)
        });
        if all_same_hospital {
            self.hospital_name = first_hospital_name;
            self.hospital_level = first.hospital_level;
        } else {
            self.hospital_name = Some("其他".to_owned());
            self.hospital_level = self
                .medicals
                .iter()
                .filter_map(|item| item.hospital_level)
                .max();
        }
        self.disease_code = first.disease.as_ref().map(|disease| disease.code.clone());
    }
}
