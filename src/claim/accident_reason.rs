// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Causes of the covered event asserted in an insurance claim.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the event cause used to assess a claimant's insurance coverage.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccidentReason {
    /// Disease caused the insured event.
    Disease,
    /// An accident caused the insured event.
    Accident,
    /// Childbirth caused the insured event.
    Birth,
    /// Another cause produced the insured event.
    Other,
}
