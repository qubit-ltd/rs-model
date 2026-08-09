// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
//! Software inventory entries reported by a device.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::system::Platform;
/// Identifies an installed operating system, application, or other software component.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Software {
    /// Persisted software-record identifier; the default value denotes no record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Software code, usually its package or bundle name; it need not be unique.
    pub code: String,

    /// Product display name; it need not be unique.
    pub name: String,

    /// Operating-system platform on which the software runs.
    pub platform: Platform,

    /// Released software version.
    pub version: String,

    /// Optional build identifier.
    pub build: Option<String>,

    /// Optional patch-level identifier.
    pub patch: Option<String>,

    /// Optional internal code name.
    pub code_name: Option<String>,

    /// Optional software vendor name.
    pub manufacturer: Option<String>,

    /// Optional descriptive text.
    pub description: Option<String>,
}
