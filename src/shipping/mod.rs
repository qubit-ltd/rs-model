// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shipment parties, carrier records, and recipient delivery constraints.

mod consign_info;
mod packing;
#[allow(clippy::module_inception)]
mod shipping;
mod shipping_demand;
mod shipping_mode;

pub use consign_info::ConsignInfo;
pub use packing::Packing;
pub use shipping::Shipping;
pub use shipping_demand::ShippingDemand;
pub use shipping_mode::ShippingMode;
