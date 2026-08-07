// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Typed system settings and their stable wire adapters.

mod data_type;
#[allow(clippy::module_inception)]
mod setting;
mod setting_adapter_error;
mod setting_json_deserializer;
mod setting_json_serializer;
mod setting_name;
mod setting_randomizer;
mod setting_xml_adapted;
mod setting_xml_adapter;

pub use data_type::DataType;
pub use setting::Setting;
pub(crate) use setting::data_type_source_name;
pub(crate) use setting::parse_data_type_name;
pub use setting_adapter_error::SettingAdapterError;
pub use setting_json_deserializer::SettingJsonDeserializer;
pub use setting_json_serializer::SettingJsonSerializer;
pub use setting_name::SettingName;
pub use setting_randomizer::SettingRandomizer;
pub use setting_xml_adapted::SettingXmlAdapted;
pub use setting_xml_adapter::SettingXmlAdapter;
