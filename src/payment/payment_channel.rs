// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment channel classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Identifies the provider rail used for a payment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
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
    /// UnionPay.
    UnionPay,
    /// Apple Pay.
    ApplePay,
    /// China Merchants Bank wallet.
    CmbWallet,
    /// Direct bank payment.
    Bank,
    /// Stored balance.
    Balance,
    /// Medicare payment.
    Medicare,
    /// Unknown channel.
    Unknown,
}
