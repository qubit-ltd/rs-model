// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verification-code usage scenarios.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// The operation for which a verification code was issued.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifyScene {
    /// Account registration.
    Register,
    /// Password reset.
    ResetPassword,
    /// Payment confirmation.
    Pay,
    /// Refund confirmation.
    Refund,
    /// Mobile-number verification.
    VerifyMobile,
    /// Email-address verification.
    VerifyEmail,
    /// Real-name verification.
    VerifyRealname,
    /// Drug collection.
    ReceiveDrug,
    /// Profile modification.
    Modify,
    /// Account login.
    Login,
    /// Employee-record binding.
    BindEmployee,
    /// Person-record binding.
    BindPerson,
    /// Any other verification scenario.
    Other,
}

impl VerifyScene {
    /// Parses the exact uppercase wire name used by the Java enumeration.
    ///
    /// Returns `None` when `name` is not one of the source enumeration names.
    #[must_use]
    pub fn from_wire_name(name: &str) -> Option<Self> {
        match name {
            "REGISTER" => Some(Self::Register),
            "RESET_PASSWORD" => Some(Self::ResetPassword),
            "PAY" => Some(Self::Pay),
            "REFUND" => Some(Self::Refund),
            "VERIFY_MOBILE" => Some(Self::VerifyMobile),
            "VERIFY_EMAIL" => Some(Self::VerifyEmail),
            "VERIFY_REALNAME" => Some(Self::VerifyRealname),
            "RECEIVE_DRUG" => Some(Self::ReceiveDrug),
            "MODIFY" => Some(Self::Modify),
            "LOGIN" => Some(Self::Login),
            "BIND_EMPLOYEE" => Some(Self::BindEmployee),
            "BIND_PERSON" => Some(Self::BindPerson),
            "OTHER" => Some(Self::Other),
            _ => None,
        }
    }
}
