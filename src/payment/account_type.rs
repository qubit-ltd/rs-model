// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Kinds of accounts that can fund or receive payments.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The financial instrument or balance represented by an account record.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// A debit or bank card account.
    BankCard,
    /// A credit-card account.
    CreditCard,
    /// A passbook-based bank deposit account.
    DepositBook,
    /// An account maintained by a third-party payment provider.
    ThirdPart,
    /// An account used for settlement.
    Settlement,
    /// A stored-value or wallet balance.
    Change,
    /// A virtual-currency balance.
    Virtual,
}
