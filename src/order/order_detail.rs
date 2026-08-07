// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Expanded order query results.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::order::Order;
use crate::order::Return;
use crate::settlement::Transaction;

/// An order together with optional related transactions and returns.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderDetail {
    /// Order aggregate.
    pub order: Order,

    /// Optional related transactions.
    pub transactions: Option<Vec<Transaction>>,

    /// Optional related returns.
    pub returns: Option<Vec<Return>>,
}
