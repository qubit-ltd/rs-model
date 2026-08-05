// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Replacement marker records.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Source-compatible marker for a replacement workflow.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize,
)]
pub struct Replacement;
