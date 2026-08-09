// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical-insurance settlement records and their supporting details.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::CredentialInfo;
use crate::medical::HisInfo;
use crate::medical::MedicalPayment;
use crate::medical::MedicalSettlementItem;
use crate::medical::MedicareType;
use crate::medical::PatientInfo;
use crate::mixin::StatefulInfo;

/// A completed medical-insurance settlement with the patient, source HIS
/// encounter, payment allocation, and itemized charges that support it.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalSettlement {
    /// Optional persisted identifier inherited from the settlement model.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application namespace responsible for the settlement record.
    pub app: StatefulInfo,

    /// Organization that performed or owns the settlement.
    pub organization: StatefulInfo,

    /// Optional settlement remark.
    pub remark: Option<String>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,

    /// Medical-insurance classification.
    pub medicare: MedicareType,

    /// Patient information.
    pub patient: PatientInfo,

    /// Credential presented as the patient's medical-insurance card.
    pub card: CredentialInfo,

    /// Encounter payload from the hospital-information system; its tagged
    /// variant identifies the source care setting.
    #[model(opaque)]
    pub his_info: HisInfo,

    /// Medical payment breakdown.
    pub payment: MedicalPayment,

    /// Non-empty itemized charges from which the settlement can be audited.
    #[model(sequence(min_items = 1))]
    pub items: Vec<MedicalSettlementItem>,
}
