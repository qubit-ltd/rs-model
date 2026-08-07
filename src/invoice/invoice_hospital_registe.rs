// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Hospital invoice-platform registrations.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Hospital registration and payment credentials for an invoice platform.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct InvoiceHospitalRegiste {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Organization code.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub code: String,

    /// Organization name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Administrative-zone code.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub zone_code: String,

    /// Optional full payee name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub payee_name: Option<String>,

    /// Optional payee bank account.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub payee_account: Option<String>,

    /// Optional payee bank name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub payee_opening_bank: Option<String>,

    /// Optional invoice-platform administrator account.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub account: Option<String>,

    /// Optional invoice-platform administrator password.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub password: Option<String>,

    /// Optional UTC platform-registration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub register_time: Option<DateTime<Utc>>,

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
