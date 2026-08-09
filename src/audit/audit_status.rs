// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States used to track the lifecycle of an audit request.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Lifecycle state assigned to a request for a domain-object review.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditStatus {
    /// The request is awaiting review after submission.
    Submitted,
}
