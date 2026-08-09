// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Error details returned when an external SMS provider cannot send a message.

use serde::Deserialize;
use std::collections::BTreeMap;
use thiserror::Error;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::error::ErrorType;
use crate::notification::NotificationErrorCode;

/// A failed SMS send directed to one recipient or a batch of recipients.
#[derive(Model, Redact, Clone, Deserialize, Error, PartialEq)]
#[redact(debug, serde)]
#[error("failed to send SMS: {third_party_message}")]
pub struct SendSmsException {
    /// Sole destination for a single-recipient send, or `None` for a batch.
    #[redact(nested)]
    pub phone: Option<Phone>,

    /// Batch destinations, or `None` when the failure concerns one recipient.
    #[redact(nested)]
    pub phones: Option<Vec<Phone>>,

    /// Provider-defined code that explains the delivery failure.
    #[redact(level = "secret")]
    pub third_party_code: String,

    /// Provider-defined diagnostic text for the delivery failure.
    #[redact(level = "secret")]
    pub third_party_message: String,
}

impl SendSmsException {
    /// Creates a delivery error for one destination and its provider response.
    #[must_use]
    pub fn for_phone(phone: Phone, third_party_code: String, third_party_message: String) -> Self {
        Self {
            phone: Some(phone),
            phones: None,
            third_party_code,
            third_party_message,
        }
    }

    /// Creates a delivery error for a recipient batch and its provider response.
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

    /// Returns the stable application error code for this failure.
    #[must_use]
    pub const fn code(&self) -> NotificationErrorCode {
        NotificationErrorCode::SendSmsFailed
    }

    /// Returns the platform category identifying a third-party failure.
    #[must_use]
    pub const fn error_type(&self) -> ErrorType {
        self.code().error_type()
    }

    /// Returns the provider diagnostic inserted as the localized message reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.third_party_message
    }

    /// Builds named message-template parameters, including a formatted recipient list.
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
