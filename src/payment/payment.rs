// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment execution records.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{
    commons::Currency,
    payment::{PaymentChannel, PaymentMode, PaymentType},
    system::Environment,
};

/// The provider-side execution and outcome of a payment transaction.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Payment {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Payment business classification.
    pub r#type: PaymentType,
    /// Persisted order identifier.
    pub order_id: i64,
    /// Persisted transaction identifier.
    pub transaction_id: i64,
    /// Payment provider application information.
    #[model(opaque)]
    pub provider_app: Info,
    /// Provider payment channel.
    pub channel: PaymentChannel,
    /// Provider interaction mode.
    pub mode: PaymentMode,
    /// Platform payment number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,
    /// Optional provider payment number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub channel_number: Option<String>,
    /// Optional raw provider reply.
    #[redact(level = "secret")]
    pub channel_reply: Option<String>,
    /// Payment currency.
    pub currency: Currency,
    /// Amount due.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,
    /// Optional discount amount.
    #[model(money(scale = 4))]
    pub discount: Option<BigDecimal>,
    /// Amount paid.
    #[model(money(scale = 4))]
    pub paid: BigDecimal,
    /// Optional provider cost.
    #[model(money(scale = 4))]
    pub cost: Option<BigDecimal>,
    /// Optional client environment.
    pub environment: Option<Environment>,
    /// Whether provider execution succeeded.
    pub success: bool,
    /// Optional structured provider error information.
    #[model(opaque)]
    pub error: Option<serde_json::Value>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: DateTime<Utc>,
    /// UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: DateTime<Utc>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
