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
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Software {
    #[model(identifier)]
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub platform: Platform,
    pub version: String,
    pub build: Option<String>,
    pub patch: Option<String>,
    pub code_name: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
}
