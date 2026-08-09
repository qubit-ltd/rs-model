// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Legal subject types for payment parties.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Whether a payer or payee is an individual or a legal organization.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParticipantType {
    /// A natural person.
    Person,
    /// A company, institution, or other legal organization.
    Organization,
}
