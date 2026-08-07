// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical settlement records.

use chrono::DateTime;
use chrono::Utc;
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

/// A settlement augmented with patient, insurance, HIS, and charge details.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct MedicalSettlement {
    /// Optional persisted identifier inherited from the settlement model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Application that owns this settlement.
    pub app: StatefulInfo,

    /// Organization that owns this settlement.
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

    /// Medical card used by the patient.
    pub card: CredentialInfo,

    /// Polymorphic hospital-information-system payload.
    #[model(opaque)]
    pub his_info: HisInfo,

    /// Medical payment breakdown.
    pub payment: MedicalPayment,

    /// Settlement line items.
    #[model(sequence(min_items = 1))]
    pub items: Vec<MedicalSettlementItem>,
}
