// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for setting JSON deserialization.

use qubit_model::setting::DataType;
use qubit_model::setting::SettingAdapterError;
use qubit_model::setting::SettingJsonDeserializer;

/// Applies absent-field defaults and canonicalizes JSON scalar values.
#[test]
fn test_deserialize_applies_defaults_and_converts_values() {
    let setting =
        SettingJsonDeserializer::deserialize(r#"{"name":"limit","values":[1,true,null,"three"]}"#)
            .expect("a JSON object with omitted optional fields is valid");

    assert_eq!(setting.name, "limit");
    assert_eq!(setting.data_type, DataType::String);
    assert_eq!(setting.values, ["1", "true", "three"]);
    assert!(setting.nullable);
    assert!(setting.multiple);
}

/// Rejects malformed roots, unsupported data types, and invalid timestamps.
#[test]
fn test_deserialize_rejects_invalid_input_variants() {
    assert!(matches!(
        SettingJsonDeserializer::deserialize("not JSON"),
        Err(SettingAdapterError::InvalidJson(_))
    ));
    assert!(matches!(
        SettingJsonDeserializer::deserialize("[]"),
        Err(SettingAdapterError::InvalidJsonRoot)
    ));
    assert!(matches!(
        SettingJsonDeserializer::deserialize(r#"{"type":"NOPE"}"#),
        Err(SettingAdapterError::InvalidDataType(value)) if value == "NOPE"
    ));
    assert!(matches!(
        SettingJsonDeserializer::deserialize(r#"{"createTime":"not-a-time"}"#),
        Err(SettingAdapterError::InvalidTimestamp(_))
    ));
}
