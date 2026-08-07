// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Replacement marker records.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Source-compatible marker for a replacement workflow.
#[derive(Model, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Replacement;
