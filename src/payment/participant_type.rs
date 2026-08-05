// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment participant classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Identifies whether a payment participant is a person or organization.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParticipantType {
    /// A natural person.
    Person,
    /// A legal organization.
    Organization,
}
