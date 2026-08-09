// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Provider rails used to execute payments.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The payment provider or internal balance rail that processed a charge.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentChannel {
    /// Alipay.
    Alipay,
    /// WeChat Pay.
    WechatPay,
    /// QQ Pay.
    QqPay,
    /// Baidu Pay.
    BaiduPay,
    /// JD Pay.
    JdPay,
    /// PayPal.
    Paypal,
    /// China UnionPay.
    UnionPay,
    /// Apple Pay.
    ApplePay,
    /// China Merchants Bank wallet.
    CmbWallet,
    /// A direct bank-transfer rail.
    Bank,
    /// An internal stored balance.
    Balance,
    /// A Medicare settlement rail.
    Medicare,
    /// A channel that was not identified by the source system.
    Unknown,
}
