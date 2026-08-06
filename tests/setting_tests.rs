// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use chrono::{TimeZone, Utc};
use qubit_model::setting::{
    DataType, Setting, SettingAdapterError, SettingJsonDeserializer, SettingName,
    SettingRandomizer, SettingXmlAdapted,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn setting_preserves_source_defaults_and_model_metadata() {
    assert_redact::<DataType>();
    assert_redact::<Setting>();

    let setting = Setting::new("feature.flags", DataType::String);

    assert!(!setting.readonly);
    assert!(setting.nullable);
    assert!(setting.multiple);

    let explicit = SettingJsonDeserializer::deserialize(
        r#"{"name":"explicit","type":"int","readonly":true,"nullable":false,"multiple":false,"encrypted":true,"description":"configured","createTime":"2025-01-02T03:04:05Z","modifyTime":"2025-01-03T03:04:05Z"}"#,
    )
    .expect("complete setting JSON is valid");
    assert_eq!(explicit.data_type, DataType::Int);
    assert!(explicit.readonly);
    assert!(!explicit.nullable);
    assert!(!explicit.multiple);
    assert!(explicit.encrypted);
    assert_eq!(explicit.description.as_deref(), Some("configured"));
    assert!(explicit.create_time.is_some());
    assert!(explicit.modify_time.is_some());
    assert!(!setting.encrypted);
    assert!(setting.is_valid());
    assert_eq!(metadata_of::<Setting>().fields().count(), 10);
}

#[test]
fn setting_round_trips_string_values_with_the_source_delimiter() {
    let mut setting = Setting::new("messages", DataType::String);
    setting.values = vec!["first".into(), String::new(), "a,b".into()];

    let persistent = setting.persistent_value().unwrap();
    assert_eq!(
        persistent,
        format!(
            "first{}{}{}a,b",
            Setting::STRING_DELIMITER,
            "",
            Setting::STRING_DELIMITER
        )
    );

    setting.values.clear();
    setting.set_persistent_value(Some(&persistent));
    assert_eq!(setting.values, ["first", "", "a,b"]);
}

#[test]
fn setting_round_trips_non_string_values_and_validates_cardinality() {
    let mut setting = Setting::new("retries", DataType::Int);
    setting.set_persistent_value(Some("1,2,"));
    assert_eq!(setting.values, ["1", "2", ""]);
    assert_eq!(setting.persistent_value().as_deref(), Some("1,2,"));

    setting.multiple = false;
    assert!(!setting.is_valid());
    setting.values.truncate(1);
    assert!(setting.is_valid());
    setting.values.clear();
    setting.nullable = false;
    assert!(!setting.is_valid());
}

#[test]
fn setting_uses_source_json_shape_and_case_insensitive_ordering() {
    let mut setting = Setting::new("Beta", DataType::Bool);
    setting.values.push("true".into());
    setting.create_time = Some(Utc.with_ymd_and_hms(2025, 1, 2, 3, 4, 5).unwrap());

    let json = serde_json::to_value(&setting).unwrap();
    assert_eq!(json["type"], "BOOL");
    assert_eq!(json["createTime"], "2025-01-02T03:04:05Z");
    assert_eq!(json["values"][0], "true");
    assert!(Setting::new("alpha", DataType::String) < setting);
    assert_eq!(
        Setting::new("BETA", DataType::String).cmp(&setting),
        std::cmp::Ordering::Equal
    );
}

#[test]
fn setting_names_match_the_java_contract() {
    assert_eq!(
        SettingName::AUDIT_LOG_REQUEST_BODY_ENABLED,
        "audit.log.request_body.enabled"
    );
    assert_eq!(SettingName::LOGIN_SESSION_MAX_AGE, "login.session.timeout");
    assert_eq!(
        SettingName::USER_MOBILE_NUMBER_REGEX,
        "user.mobile.number.regex"
    );
}

#[test]
fn setting_json_deserialization_applies_defaults_and_reports_invalid_inputs() {
    assert_eq!(SettingJsonDeserializer::default(), SettingJsonDeserializer);
    let setting =
        SettingJsonDeserializer::deserialize(r#"{"name":"limit","values":[1,true,null,"three"]}"#)
            .expect("a JSON object with omitted optional fields is valid");
    assert_eq!(setting.name, "limit");
    assert_eq!(setting.data_type, DataType::String);
    assert_eq!(setting.values, ["1", "true", "three"]);
    assert!(setting.nullable);
    assert!(setting.multiple);

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

#[test]
fn setting_xml_transfer_and_seeded_randomizer_preserve_valid_contracts() {
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
    assert!(matches!(
        SettingXmlAdapted { type_name: Some("unknown".into()), ..SettingXmlAdapted::default() }.to_setting(),
        Err(SettingAdapterError::InvalidDataType(value)) if value == "unknown"
    ));

    let mut first = SettingRandomizer::with_seed(42);
    let mut second = SettingRandomizer::with_seed(42);
    first.set_collection_size_range(2, 2);
    second.set_collection_size_range(2, 2);
    first.set_string_length_range(3, 3);
    second.set_string_length_range(3, 3);
    let generated = first.get();
    assert_eq!(generated, second.get());
    assert!(generated.is_valid());
    assert_eq!(generated.values.len(), 2);
    assert!(SettingRandomizer::SUPPORTED_TYPES.contains(&generated.data_type));
}

#[test]
fn setting_randomizer_rejects_invalid_ranges() {
    let mut randomizer = SettingRandomizer::default();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            randomizer.set_collection_size_range(2, 1);
        }))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            randomizer.set_string_length_range(0, 1);
        }))
        .is_err()
    );
}

#[test]
fn setting_value_conversion_covers_all_source_data_types() {
    let data_types = [
        DataType::Bool,
        DataType::Char,
        DataType::Byte,
        DataType::Short,
        DataType::Int,
        DataType::Long,
        DataType::Float,
        DataType::Double,
        DataType::String,
        DataType::Date,
        DataType::Time,
        DataType::Datetime,
        DataType::Instant,
        DataType::Timestamp,
        DataType::ByteArray,
        DataType::Class,
        DataType::BigInteger,
        DataType::BigDecimal,
        DataType::StringArray,
        DataType::Enum,
        DataType::EnumArray,
    ];
    for data_type in data_types {
        let mut setting = Setting::new("typed", data_type);
        setting.values = vec!["one".into(), "two".into()];
        let adapted = SettingXmlAdapted::from_setting(&setting);
        assert_eq!(
            adapted.to_setting().expect("known type round trips"),
            setting
        );
    }

    let mut setting = Setting::default();
    assert_eq!(setting.persistent_value(), None);
    setting.set_persistent_value(Some("one"));
    assert_eq!(setting.persistent_value().as_deref(), Some("one"));
    setting.set_persistent_value(None);
    assert!(setting.values.is_empty());
    assert_eq!(
        setting.partial_cmp(&Setting::new("other", DataType::String)),
        Some(std::cmp::Ordering::Less)
    );

    let mut randomizer = SettingRandomizer::with_seed(0);
    for _ in 0..512 {
        let setting = randomizer.get();
        assert!(setting.is_valid());
    }
}
