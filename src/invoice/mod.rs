// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice records, applications, stock, and classifications.

#[allow(clippy::module_inception)]
mod invoice;
mod invoice_apply;
mod invoice_apply_status;
mod invoice_hospital_registe;
mod invoice_info;
mod invoice_item;
mod invoice_number_segment;
mod invoice_place;
mod invoice_status;
mod invoice_stock_status;
mod invoice_title_type;
mod invoice_type;

pub use invoice::Invoice;
pub use invoice_apply::InvoiceApply;
pub use invoice_apply_status::InvoiceApplyStatus;
pub use invoice_hospital_registe::InvoiceHospitalRegiste;
pub use invoice_info::InvoiceInfo;
pub use invoice_item::InvoiceItem;
pub use invoice_number_segment::InvoiceNumberSegment;
pub use invoice_place::InvoicePlace;
pub use invoice_status::InvoiceStatus;
pub use invoice_stock_status::InvoiceStockStatus;
pub use invoice_title_type::InvoiceTitleType;
pub use invoice_type::InvoiceType;
