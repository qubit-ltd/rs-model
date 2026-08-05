use chrono::{TimeZone, Utc};
use qubit_model::setting::{DataType, Setting, SettingName};
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
