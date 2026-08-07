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

/// Describes the lifecycle of a request.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
    /// Newly created.
    Created,
    /// Submitted for processing.
    Submitted,
    /// Waiting for processing.
    Pending,
    /// Being processed.
    Processing,
    /// Processing failed.
    Failed,
    /// Cancelled before completion.
    Cancelled,
    /// Completed successfully.
    Completed,
}

impl RequestStatus {
    /// Returns the Java-compatible serialized name.
    ///
    /// # Returns
    ///
    /// The screaming-snake-case status name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "CREATED",
            Self::Submitted => "SUBMITTED",
            Self::Pending => "PENDING",
            Self::Processing => "PROCESSING",
            Self::Failed => "FAILED",
            Self::Cancelled => "CANCELLED",
            Self::Completed => "COMPLETED",
        }
    }
}
