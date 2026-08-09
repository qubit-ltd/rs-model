// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! In-place filtering that produces the transaction view safe for a payment gateway.

use qubit_id::Id;
use qubit_model_derive::Model;

use crate::settlement::Transaction;

/// Stateless transformer that strips settlement-only data from a gateway request.
#[derive(Model, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PaymentRequestTransformer;

impl PaymentRequestTransformer {
    /// Removes provider-irrelevant and internal fields from a transaction in place.
    ///
    /// Fields required to identify the payer and execute the charge are retained; the input is
    /// permanently mutated.
    ///
    /// # Parameters
    ///
    /// * `transaction` - Transaction to filter in place.
    pub fn transform(transaction: &mut Transaction) {
        transaction.r#type = None;
        transaction.origin_id = Id::default();
        transaction.return_id = Id::default();
        transaction.status = None;
        transaction.app = None;
        transaction.source = None;
        transaction.category = None;
        transaction.discount = None;
        transaction.paid = None;
        transaction.payee = None;
        transaction.payment = None;
        transaction.complete_time = None;
        transaction.invoice_status = None;
        transaction.comment = None;
        transaction.environment = None;
        transaction.modify_time = None;
        transaction.delete_time = None;

        transaction.payer.id = Id::default();
        transaction.payer.r#type = None;
        transaction.payer.phone = None;
        transaction.payer.email = None;
        transaction.payer.account = None;
    }
}
