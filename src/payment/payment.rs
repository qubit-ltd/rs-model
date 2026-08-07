// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment execution records.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Currency;
use crate::payment::PaymentChannel;
use crate::payment::PaymentMode;
use crate::payment::PaymentType;
use crate::system::Environment;

/// The provider-side execution and outcome of a payment transaction.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Payment {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Payment business classification.
    pub r#type: PaymentType,

    /// Persisted order identifier.
    #[model(opaque)]
    pub order_id: Id,

    /// Persisted transaction identifier.
    #[model(opaque)]
    pub transaction_id: Id,

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
