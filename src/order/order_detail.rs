// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Expanded order query results.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    order::{
        Order,
        Return,
    },
    settlement::Transaction,
};

/// An order together with optional related transactions and returns.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct OrderDetail {
    /// Order aggregate.
    pub order: Order,
    /// Optional related transactions.
    pub transactions: Option<Vec<Transaction>>,
    /// Optional related returns.
    pub returns: Option<Vec<Return>>,
}
