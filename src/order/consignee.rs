// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Saved order consignee records.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::CredentialInfo;
use crate::contact::Address;
use crate::contact::Phone;
use crate::person::User;

/// A saved recipient and delivery address for an order.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Consignee {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Identifier of the owning-user; its default value means that no related record is stored.
    #[model(reference(target = User, target_field = id), opaque)]
    pub user_id: Id,

    /// Optional saved-address title.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Recipient name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Recipient mobile number.
    pub mobile: Phone,

    /// Optional recipient email address.
    #[model(text(min_chars = 1, max_chars = 512))]
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Optional identity credential.
    pub credential: Option<CredentialInfo>,

    /// Delivery address.
    pub address: Address,

    /// Recipient comment.
    pub comment: String,

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
