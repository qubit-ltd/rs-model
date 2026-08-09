// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Marker type retained for replacement-workflow compatibility.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Zero-sized marker identifying the replacement workflow in serialized models.
#[derive(Model, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Replacement;
