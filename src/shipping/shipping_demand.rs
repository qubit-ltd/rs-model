// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Recipient preferences and restrictions for delivery scheduling and handling.

use chrono::NaiveDate;
use chrono::NaiveTime;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::DayType;
use crate::shipping::Packing;

/// Delivery constraints that a carrier should honor when fulfilling an order.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ShippingDemand {
    /// Accepted day type, or `None` when any day type is acceptable.
    pub day_type: Option<DayType>,

    /// Optional half-open local-date range with independently open bounds.
    #[model(opaque)]
    pub date_range: Option<(Option<NaiveDate>, Option<NaiveDate>)>,

    /// Optional half-open local-time range with independently open bounds.
    #[model(opaque)]
    pub time_range: Option<(Option<NaiveTime>, Option<NaiveTime>)>,

    /// Packing protection required for the shipment.
    pub packing: Packing,

    /// Additional delivery instructions, or `None` when none were provided.
    pub comment: Option<String>,
}
