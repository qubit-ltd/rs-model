// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for XML-oriented setting values.

use qubit_model::setting::DataType;
use qubit_model::setting::Setting;
use qubit_model::setting::SettingAdapterError;
use qubit_model::setting::SettingXmlAdapted;

/// Omits source defaults and reconstructs a complete setting.
#[test]
fn test_xml_adapted_round_trips_defaults_and_overrides() {
    let mut setting = Setting::new("demo", DataType::Int);
    setting.readonly = true;
    setting.multiple = false;
    setting.values = vec!["4".into()];

    let adapted = SettingXmlAdapted::from_setting(&setting);
    assert_eq!(adapted.type_name.as_deref(), Some("int"));
    assert_eq!(adapted.readonly, Some(true));
    assert_eq!(adapted.nullable, None);
    assert_eq!(
        adapted.to_setting().expect("adapted setting is valid"),
        setting
    );
}

/// Rejects an unsupported XML data type without discarding the source value.
#[test]
fn test_xml_adapted_rejects_unsupported_type() {
    let adapted = SettingXmlAdapted {
        type_name: Some("unknown".into()),
        ..SettingXmlAdapted::default()
    };
    assert!(matches!(
        adapted.to_setting(),
        Err(SettingAdapterError::InvalidDataType(value)) if value == "unknown"
    ));
}
