// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::setting::{
    DataType,
    Setting,
    SettingJsonDeserializer,
    SettingJsonSerializer,
    SettingRandomizer,
    SettingXmlAdapted,
    SettingXmlAdapter,
};
use qubit_model::system::{
    SettingJsonDeserializer as SystemSettingJsonDeserializer,
    SettingJsonSerializer as SystemSettingJsonSerializer,
    SettingRandomizer as SystemSettingRandomizer,
    SettingXmlAdapted as SystemSettingXmlAdapted,
    SettingXmlAdapter as SystemSettingXmlAdapter,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires diagnostic redaction for the public XML transfer value.
fn assert_redact<T: Redact>() {}

#[test]
fn test_setting_json_adapters_preserve_defaults_and_values() {
    let decoded = SettingJsonDeserializer::deserialize(
        r#"{"name":"login.failure.max","type":"int","values":["5"]}"#,
    )
    .expect("decode setting");
    assert_eq!(decoded.name, "login.failure.max");
    assert_eq!(decoded.data_type, DataType::Int);
    assert_eq!(decoded.values, ["5"]);
    assert!(!decoded.readonly);
    assert!(decoded.nullable);
    assert!(decoded.multiple);
    assert!(!decoded.encrypted);

    let encoded =
        SettingJsonSerializer::serialize(&decoded).expect("encode setting");
    let value: serde_json::Value =
        serde_json::from_str(&encoded).expect("setting JSON");
    assert_eq!(value["type"], "INT");
    assert_eq!(value["values"][0], "5");
}

#[test]
fn test_setting_xml_adapter_omits_defaults_and_round_trips() {
    let mut setting = Setting::new("system.time_zone", DataType::String);
    setting.readonly = true;
    setting.description = Some("System time zone".into());
    setting.values = vec!["Asia/Shanghai".into()];

    let adapted =
        SettingXmlAdapter::marshal(Some(&setting)).expect("adapted setting");
    assert_eq!(adapted.name, "system.time_zone");
    assert_eq!(adapted.type_name, None);
    assert_eq!(adapted.readonly, Some(true));
    assert_eq!(adapted.nullable, None);
    assert_eq!(
        adapted
            .values
            .as_deref()
            .and_then(|values| values.first())
            .map(String::as_str),
        Some("Asia/Shanghai")
    );

    let decoded = SettingXmlAdapter::unmarshal(Some(&adapted))
        .expect("unmarshal setting")
        .expect("setting value");
    assert_eq!(decoded, setting);
}

#[test]
fn test_setting_xml_adapted_is_a_model_and_redactable() {
    assert_redact::<SettingXmlAdapted>();
    assert_eq!(metadata_of::<SettingXmlAdapted>().struct_fields().len(), 10);
}

#[test]
fn test_setting_adapters_are_exported_from_the_source_system_package() {
    let _ = SystemSettingJsonDeserializer;
    let _ = SystemSettingJsonSerializer;
    let _ = SystemSettingRandomizer::with_seed(1);
    let _ = SystemSettingXmlAdapted::default();
    let _ = SystemSettingXmlAdapter;
}

#[test]
fn test_setting_randomizer_is_seeded_and_always_generates_valid_settings() {
    let mut first = SettingRandomizer::with_seed(42);
    let mut second = SettingRandomizer::with_seed(42);
    first.set_collection_size_range(0, 3);
    second.set_collection_size_range(0, 3);

    for _ in 0..32 {
        let left = first.get();
        let right = second.get();
        assert_eq!(left, right);
        assert!(left.name.starts_with("setting_"));
        assert!(left.create_time.is_some());
        assert!(left.is_valid());
        assert_eq!(left.multiple, left.values.len() > 1);
    }
}
