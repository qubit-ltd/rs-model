// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment account classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Identifies the kind of a payment account.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// Bank card account.
    BankCard,
    /// Credit card account.
    CreditCard,
    /// Bank deposit book.
    DepositBook,
    /// Third-party payment account.
    ThirdPart,
    /// Settlement account.
    Settlement,
    /// Change or wallet account.
    Change,
    /// Virtual-currency account.
    Virtual,
}
