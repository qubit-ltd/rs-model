// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Return reason classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Explains why an order item is being returned.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnReason {
    /// No reason was supplied.
    NoReason,
    /// The buyer cannot complete the purchase.
    BuyerIncapable,
    /// The item is out of stock.
    OutOfStock,
    /// The item is no longer sold.
    StopSelling,
    /// The buyer dislikes the item.
    Dislike,
    /// The buyer no longer wants the item.
    DontWant,
    /// The purchase was accidental.
    Misoperation,
    /// Parts are missing.
    LackOfPart,
    /// The product is broken.
    BrokenProduct,
    /// The wrong product was supplied.
    WrongProduct,
    /// The product is expired.
    ExpiredProduct,
    /// The product has quality problems.
    QualityProblems,
    /// The product differs from its description.
    MismatchDescription,
    /// The product is too expensive.
    TooExpensive,
    /// The product is an imitation.
    Imitation,
}
