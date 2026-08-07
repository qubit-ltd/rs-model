// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Return lifecycle states.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes a return request's lifecycle state.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnStatus {
    /// The return expired.
    Expired,
    /// The return was cancelled.
    Cancelled,
    /// The return was submitted.
    Submitted,
    /// The return was accepted.
    Accepted,
    /// The return was rejected.
    Rejected,
    /// The returned item was sent.
    Sent,
    /// The returned item was received.
    Received,
    /// The return completed.
    Completed,
    /// A refund is being processed.
    Refunding,
    /// The refund failed.
    RefundFail,
    /// The refund succeeded.
    RefundSuccess,
}

impl ReturnStatus {
    /// Returns whether this state ends the return workflow.
    ///
    /// # Returns
    ///
    /// `true` for terminal states; otherwise, `false`.
    #[must_use]
    pub const fn is_finished(self) -> bool {
        matches!(
            self,
            Self::Expired
                | Self::Cancelled
                | Self::Rejected
                | Self::Completed
                | Self::RefundFail
                | Self::RefundSuccess
        )
    }
}
