// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Registered-device inventory, hardware details, and telemetry models.

mod data_network_type;
#[allow(clippy::module_inception)]
mod device;
mod device_current_data;
mod device_info;
mod device_type;
mod hardware;
mod sim_card;
mod sim_card_status;
mod software;

pub use data_network_type::DataNetworkType;
pub use device::Device;
pub use device_current_data::DeviceCurrentData;
pub use device_info::DeviceInfo;
pub use device_type::DeviceType;
pub use hardware::Hardware;
pub use sim_card::SimCard;
pub use sim_card_status::SimCardStatus;
pub use software::Software;
