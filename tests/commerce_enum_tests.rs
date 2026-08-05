// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for order, payment, and settlement classifications.

use qubit_model::{
    order::{
        ConfirmStatus,
        OpenidType,
        OrderStatus,
        PayType,
        RefererOrderRecordStatus,
        ReturnIssuer,
        ReturnReason,
        ReturnStatus,
    },
    payment::{
        AccountType,
        ParticipantType,
        PaymentChannel,
        PaymentMode,
        PaymentOption,
        PaymentType,
    },
    settlement::{
        TransactionStatus,
        TransactionType,
    },
};

/// Verifies representative commerce classifications preserve Java wire names.
#[test]
fn test_commerce_enums_preserve_java_wire_values() {
    let cases = [
        serde_json::to_string(&OrderStatus::PaidSuccess)
            .expect("order status should serialize"),
        serde_json::to_string(&ReturnReason::MismatchDescription)
            .expect("return reason should serialize"),
        serde_json::to_string(&PaymentChannel::WechatPay)
            .expect("payment channel should serialize"),
        serde_json::to_string(&PaymentMode::ActiveQr)
            .expect("payment mode should serialize"),
        serde_json::to_string(&TransactionType::Refund)
            .expect("transaction type should serialize"),
    ];

    assert_eq!(
        cases,
        [
            "\"PAID_SUCCESS\"",
            "\"MISMATCH_DESCRIPTION\"",
            "\"WECHAT_PAY\"",
            "\"ACTIVE_QR\"",
            "\"REFUND\"",
        ]
    );
}

/// Verifies all migrated classifications remain publicly reachable.
#[test]
fn test_commerce_enum_public_paths_are_complete() {
    let _ = ConfirmStatus::Accepted;
    let _ = OpenidType::Esign;
    let _ = PayType::Medicare;
    let _ = RefererOrderRecordStatus::Refund;
    let _ = ReturnIssuer::Platform;
    let _ = AccountType::Settlement;
    let _ = ParticipantType::Organization;
    let _ = PaymentOption::PaidByMedicare;
    let _ = PaymentType::Normal;
    let _ = TransactionStatus::Success;
}

/// Verifies terminal return states retain their source-domain semantics.
#[test]
fn test_return_status_reports_finished_states() {
    assert!(ReturnStatus::Completed.is_finished());
    assert!(ReturnStatus::RefundFail.is_finished());
    assert!(!ReturnStatus::Submitted.is_finished());
    assert!(!ReturnStatus::Refunding.is_finished());
}
