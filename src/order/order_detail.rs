// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Expanded query view joining an order to its settlements and returns.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::order::Order;
use crate::order::Return;
use crate::settlement::Transaction;

/// An order aggregate with optionally loaded transaction and return collections.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderDetail {
    /// The order aggregate for this detail view.
    pub order: Order,

    /// Related settlement transactions, or `None` when transactions were not included in the query.
    pub transactions: Option<Vec<Transaction>>,

    /// Related return requests, or `None` when returns were not included in the query.
    pub returns: Option<Vec<Return>>,
}
