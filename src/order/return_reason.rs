// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Buyer and seller reasons for returning an order item.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The business reason recorded when an order item enters the return workflow.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnReason {
    /// No specific reason was recorded.
    NoReason,
    /// The buyer is unable to complete the purchase.
    BuyerIncapable,
    /// The item became unavailable because it is out of stock.
    OutOfStock,
    /// The seller no longer offers the item.
    StopSelling,
    /// The buyer is dissatisfied with the item.
    Dislike,
    /// The buyer no longer wants the item.
    DontWant,
    /// The purchase was made by mistake.
    Misoperation,
    /// The delivered item is missing required parts.
    LackOfPart,
    /// The delivered product is damaged or unusable.
    BrokenProduct,
    /// The delivered product differs from the item ordered.
    WrongProduct,
    /// The product has passed its expiry date.
    ExpiredProduct,
    /// The product has a quality defect.
    QualityProblems,
    /// The product does not match its listing description.
    MismatchDescription,
    /// The charged price is considered excessive.
    TooExpensive,
    /// The product is suspected to be counterfeit.
    Imitation,
}
