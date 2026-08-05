// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment-request transaction filtering.

use qubit_model_derive::Model;

use crate::settlement::Transaction;

/// Removes internal settlement data before a transaction leaves the service.
#[derive(Clone, Copy, Debug, Default, Eq, Model, PartialEq)]
pub struct PaymentRequestTransformer;

impl PaymentRequestTransformer {
    /// Removes provider-irrelevant and internal fields from a transaction.
    ///
    /// Fields needed to identify and execute the transaction remain unchanged.
    ///
    /// # Parameters
    ///
    /// * `transaction` - Transaction to filter in place.
    pub fn transform(transaction: &mut Transaction) {
        transaction.r#type = None;
        transaction.origin_id = None;
        transaction.return_id = None;
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

        transaction.payer.id = None;
        transaction.payer.r#type = None;
        transaction.payer.phone = None;
        transaction.payer.email = None;
        transaction.payer.account = None;
    }
}
