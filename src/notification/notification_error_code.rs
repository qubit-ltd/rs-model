//! Error codes emitted by the notification domain.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::error::ErrorType;

/// A stable notification-module error code.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationErrorCode {
    /// Sending an SMS through a third-party provider failed.
    SendSmsFailed,
}

impl NotificationErrorCode {
    /// Returns the broad platform error category for this code.
    #[must_use]
    pub const fn error_type(self) -> ErrorType {
        match self {
            Self::SendSmsFailed => ErrorType::ThirdPartyError,
        }
    }

    /// Returns the Simplified Chinese message template from the Java resource bundle.
    #[must_use]
    pub const fn message_template_zh_cn(self) -> &'static str {
        match self {
            Self::SendSmsFailed => "发送短信失败：{reason}",
        }
    }
}
