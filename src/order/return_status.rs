// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States through which a return and its refund progress.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The current operational state of a return request.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnStatus {
    /// The return was not completed before its deadline.
    Expired,
    /// The return workflow was cancelled.
    Cancelled,
    /// The return awaits review or processing.
    Submitted,
    /// The return was accepted for fulfilment.
    Accepted,
    /// The return request was rejected.
    Rejected,
    /// The customer dispatched the returned item.
    Sent,
    /// The seller or carrier received the returned item.
    Received,
    /// The return workflow completed without a pending refund.
    Completed,
    /// The associated refund is in progress.
    Refunding,
    /// The refund attempt failed.
    RefundFail,
    /// The refund completed successfully.
    RefundSuccess,
}

impl ReturnStatus {
    /// Returns whether this state has no further legal return transitions.
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
