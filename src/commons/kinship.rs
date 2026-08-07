// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared enumerations from the Java commons model package.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes a kinship relationship.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Kinship {
    /// The subject themself.
    Self_,
    /// Parent relationship.
    Parent,
    /// Spouse relationship.
    Spouse,
    /// Child relationship.
    Child,
    /// Other relationship.
    Other,
}
