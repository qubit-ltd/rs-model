// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Recipient delivery requirements.

use chrono::{NaiveDate, NaiveTime};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{commons::DayType, shipping::Packing};

/// Optional scheduling, packing, and comment requirements for delivery.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct ShippingDemand {
    /// Optional accepted day classification.
    pub day_type: Option<DayType>,
    /// Optional half-open local-date range with independently open bounds.
    #[model(opaque)]
    pub date_range: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
    /// Optional half-open local-time range with independently open bounds.
    #[model(opaque)]
    pub time_range: Option<(Option<NaiveTime>, Option<NaiveTime>)>,
    /// Required packing method.
    pub packing: Packing,
    /// Optional additional delivery request.
    pub comment: Option<String>,
}
