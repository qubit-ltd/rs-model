// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Logical relations for permission expressions.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Relation joining permission expressions.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogicRelation {
    /// Logical conjunction.
    #[default]
    And,
    /// Logical disjunction.
    Or,
    /// Logical negation.
    Not,
}

impl LogicRelation {
    /// Returns the source symbol for this relation.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
        }
    }
}
