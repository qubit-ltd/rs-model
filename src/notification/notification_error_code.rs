// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Stable error codes exposed by notification operations.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::error::ErrorType;

/// Machine-readable error code for a failed notification operation.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationErrorCode {
    /// A third-party SMS provider rejected or failed the send operation.
    SendSmsFailed,
}

impl NotificationErrorCode {
    /// Returns the platform error category associated with this notification failure.
    #[must_use]
    pub const fn error_type(self) -> ErrorType {
        match self {
            Self::SendSmsFailed => ErrorType::ThirdPartyError,
        }
    }

    /// Returns the Simplified Chinese template for this code's user-facing message.
    #[must_use]
    pub const fn message_template_zh_cn(self) -> &'static str {
        match self {
            Self::SendSmsFailed => "发送短信失败：{reason}",
        }
    }
}
