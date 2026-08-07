// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use crate::system::Platform;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
/// Represents the Software domain type.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Software {
    /// The id value associated with this model.
    #[model(identifier)]
    pub id: Option<i64>,

    /// The code value associated with this model.
    pub code: String,

    /// The name value associated with this model.
    pub name: String,

    /// The platform value associated with this model.
    pub platform: Platform,

    /// The version value associated with this model.
    pub version: String,

    /// The build value associated with this model.
    pub build: Option<String>,

    /// The patch value associated with this model.
    pub patch: Option<String>,

    /// The code_name value associated with this model.
    pub code_name: Option<String>,

    /// The manufacturer value associated with this model.
    pub manufacturer: Option<String>,

    /// The description value associated with this model.
    pub description: Option<String>,
}
