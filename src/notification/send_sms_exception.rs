// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors returned when an SMS provider rejects a send operation.

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use thiserror::Error;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::error::ErrorType;
use crate::notification::NotificationErrorCode;

/// A failed single-recipient or batch SMS operation.
#[derive(
    Clone, Debug, Deserialize, Error, Model, PartialEq, Redact, Serialize,
)]
#[error("failed to send SMS: {third_party_message}")]
pub struct SendSmsException {
    /// Single destination, or `None` for a batch operation.
    #[redact(nested)]
    pub phone: Option<Phone>,

    /// Batch destinations, or `None` for a single-recipient operation.
    #[redact(nested)]
    pub phones: Option<Vec<Phone>>,

    /// Provider-specific error code.
    #[redact(level = "secret")]
    pub third_party_code: String,

    /// Provider-specific error message.
    #[redact(level = "secret")]
    pub third_party_message: String,
}

impl SendSmsException {
    /// Creates an error for a single destination phone number.
    #[must_use]
    pub fn for_phone(
        phone: Phone,
        third_party_code: String,
        third_party_message: String,
    ) -> Self {
        Self {
            phone: Some(phone),
            phones: None,
            third_party_code,
            third_party_message,
        }
    }

    /// Creates an error for a batch of destination phone numbers.
    #[must_use]
    pub fn for_phones(
        phones: Vec<Phone>,
        third_party_code: String,
        third_party_message: String,
    ) -> Self {
        Self {
            phone: None,
            phones: Some(phones),
            third_party_code,
            third_party_message,
        }
    }

    /// Returns the stable notification error code.
    #[must_use]
    pub const fn code(&self) -> NotificationErrorCode {
        NotificationErrorCode::SendSmsFailed
    }

    /// Returns the broad third-party error category.
    #[must_use]
    pub const fn error_type(&self) -> ErrorType {
        self.code().error_type()
    }

    /// Returns the provider message used as the localized template's reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.third_party_message
    }

    /// Builds the message-template parameters exposed by the Java exception.
    #[must_use]
    pub fn parameters(&self) -> BTreeMap<&'static str, Option<String>> {
        let phone = if let Some(phone) = &self.phone {
            Some(phone.to_string())
        } else {
            self.phones.as_ref().map(|phones| {
                let numbers = phones
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{numbers}]")
            })
        };
        BTreeMap::from([
            ("phone", phone),
            ("third_party_code", Some(self.third_party_code.clone())),
            (
                "third_party_message",
                Some(self.third_party_message.clone()),
            ),
            ("reason", Some(self.third_party_message.clone())),
        ])
    }
}
