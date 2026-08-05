// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order, return, referral, and checkout models.

mod buyer;
mod client;
mod client_order;
mod client_refund_submit_request;
mod confirm_status;
mod consignee;
mod openid_type;
#[allow(clippy::module_inception)]
mod order;
mod order_detail;
mod order_info;
mod order_item;
mod order_status;
mod order_submit_request;
mod order_submit_response;
mod pay_type;
mod referer_info;
mod referer_order_record_status;
mod referer_order_record;
mod replacement;
mod r#return;
mod return_issuer;
mod return_reason;
mod return_status;

pub use buyer::Buyer;
pub use client::Client;
pub use client_order::ClientOrder;
pub use client_refund_submit_request::ClientRefundSubmitRequest;
pub use confirm_status::ConfirmStatus;
pub use consignee::Consignee;
pub use openid_type::OpenidType;
pub use order::Order;
pub use order_detail::OrderDetail;
pub use order_info::OrderInfo;
pub use order_item::OrderItem;
pub use order_status::OrderStatus;
pub use order_submit_request::OrderSubmitRequest;
pub use order_submit_response::OrderSubmitResponse;
pub use pay_type::PayType;
pub use referer_info::RefererInfo;
pub use referer_order_record::RefererOrderRecord;
pub use referer_order_record_status::RefererOrderRecordStatus;
pub use replacement::Replacement;
pub use r#return::Return;
pub use return_issuer::ReturnIssuer;
pub use return_reason::ReturnReason;
pub use return_status::ReturnStatus;
