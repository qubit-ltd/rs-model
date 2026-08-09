// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Operations for which a verification token may be issued.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Business operation that a verification token authorizes.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifyScene {
    /// Creates a new account.
    Register,
    /// Resets an account password.
    ResetPassword,
    /// Confirms a payment.
    Pay,
    /// Confirms a refund.
    Refund,
    /// Verifies ownership of a mobile number.
    VerifyMobile,
    /// Verifies ownership of an email address.
    VerifyEmail,
    /// Verifies the user's real-name identity.
    VerifyRealname,
    /// Authorizes collection of prescribed medication.
    ReceiveDrug,
    /// Confirms a profile change.
    Modify,
    /// Authenticates an account login.
    Login,
    /// Binds an employee record.
    BindEmployee,
    /// Binds a person record.
    BindPerson,
    /// Covers a verification scenario not represented by a dedicated variant.
    Other,
}

impl VerifyScene {
    /// Parses an exact uppercase source-model wire name into a verification scene.
    ///
    /// Returns `None` when `name` is not a recognized wire-level scene name.
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
