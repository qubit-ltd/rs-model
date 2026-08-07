// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! China administrative-data markers and resident identity-card utilities.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;


/// Marker for the source China city dataset.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize)]
pub struct ChinaCities;
