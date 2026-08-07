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

/// Identifies the type of a failed message-queue task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MqType {
    /// A produced message.
    Produce,
}
