// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Entity classifications used by generic model references and localized messages.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Classifies the kind of domain record addressed by a generic reference.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Entity {
    /// A tenant application.
    App,
    /// A reusable classification category.
    Category,
    /// An identity credential.
    Credential,
    /// A reference-data dictionary.
    Dict,
    /// An entry belonging to a reference-data dictionary.
    DictEntry,
    /// A source channel or origin.
    Source,
    /// An auxiliary payload record.
    Payload,
    /// An authenticated session.
    Session,
    /// A one-time verification code.
    VerifyCode,
}

impl Entity {
    /// Returns the stable lowercase identifier used in model references.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Category => "category",
            Self::Credential => "credential",
            Self::Dict => "dict",
            Self::DictEntry => "dict_entry",
            Self::Source => "source",
            Self::Payload => "payload",
            Self::Session => "session",
            Self::VerifyCode => "verify_code",
        }
    }
}
