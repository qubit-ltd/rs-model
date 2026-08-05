// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for payment gateway message migrations.

use bigdecimal::BigDecimal;
use chrono::Utc;
use qubit_model::{
    commons::Currency,
    invoice::InvoiceStatus,
    mixin::StatefulInfo,
    order::ReturnIssuer,
    payment::{
        Account, AccountType, Participant, ParticipantType, PaymentRequest,
        PaymentRequestTransformer, PaymentResponse, PaymentResponseBase64,
    },
    settlement::{Transaction, TransactionStatus, TransactionType},
    system::Environment,
};
use qubit_model_metadata::metadata_of;

/// Verifies payment message structs retain every Java source field.
#[test]
fn test_payment_message_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<PaymentRequest>().struct_fields().len(), 4);
    assert_eq!(metadata_of::<PaymentRequestTransformer>().struct_fields().len(), 0);
    assert_eq!(metadata_of::<PaymentResponse>().struct_fields().len(), 4);
    assert_eq!(metadata_of::<PaymentResponseBase64>().struct_fields().len(), 1);
}

/// Verifies gateway filtering removes provider-irrelevant transaction data.
#[test]
fn test_payment_request_filter_removes_internal_fields() {
    let now = Utc::now();
    let account = Account {
        id: Some(11),
        app: StatefulInfo::default(),
        owner_type: "customer".to_owned(),
        owner_id: 12,
        r#type: AccountType::Change,
        name: "wallet".to_owned(),
        number: Some("account-number".to_owned()),
        provider: None,
        create_time: now,
        modify_time: Some(now),
        delete_time: Some(now),
    };
    let payer = Participant {
        id: Some(21),
        r#type: Some(ParticipantType::Person),
        name: "buyer".to_owned(),
        credential: None,
        mobile: None,
        phone: None,
        email: Some("buyer@example.com".to_owned()),
        account: Some(account),
        category: None,
    };
    let transaction = Transaction {
        id: Some(31),
        r#type: Some(TransactionType::Buy),
        origin_id: Some(32),
        status: Some(TransactionStatus::Submitted),
        app: Some(StatefulInfo::default()),
        source: None,
        category: None,
        order_id: 33,
        return_id: Some(34),
        return_issuer: Some(ReturnIssuer::Buyer),
        currency: Currency::Cny,
        payable: BigDecimal::from(100),
        discount: Some(BigDecimal::from(10)),
        paid: Some(BigDecimal::from(90)),
        payee: Some(payer.clone()),
        payer,
        payment: None,
        expired_time: now,
        complete_time: Some(now),
        invoice_status: Some(InvoiceStatus::NotRequired),
        environment: Some(Environment::default()),
        comment: Some("internal".to_owned()),
        create_time: now,
        modify_time: Some(now),
        delete_time: Some(now),
    };
    let mut request = PaymentRequest {
        data: transaction,
        return_url: "https://example.com/return".to_owned(),
        notify_url: "https://example.com/notify".to_owned(),
        signature: Some("signature".to_owned()),
    };

    request.filter();

    let filtered = &request.data;
    assert_eq!(filtered.id, Some(31));
    assert_eq!(filtered.order_id, 33);
    assert_eq!(filtered.return_issuer, Some(ReturnIssuer::Buyer));
    assert!(filtered.r#type.is_none());
    assert!(filtered.origin_id.is_none());
    assert!(filtered.return_id.is_none());
    assert!(filtered.status.is_none());
    assert!(filtered.app.is_none());
    assert!(filtered.discount.is_none());
    assert!(filtered.paid.is_none());
    assert!(filtered.payee.is_none());
    assert!(filtered.complete_time.is_none());
    assert!(filtered.invoice_status.is_none());
    assert!(filtered.environment.is_none());
    assert!(filtered.comment.is_none());
    assert!(filtered.modify_time.is_none());
    assert!(filtered.delete_time.is_none());
    assert!(filtered.payer.id.is_none());
    assert!(filtered.payer.r#type.is_none());
    assert!(filtered.payer.email.is_none());
    assert!(filtered.payer.account.is_none());
}
