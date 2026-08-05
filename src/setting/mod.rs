//! Typed system settings and their stable names.

use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// The value type declared by a setting.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataType {
    /// Boolean value.
    Bool,
    /// Unicode scalar value.
    Char,
    /// Signed byte value.
    Byte,
    /// Signed 16-bit integer value.
    Short,
    /// Signed 32-bit integer value.
    Int,
    /// Signed 64-bit integer value.
    Long,
    /// 32-bit floating-point value.
    Float,
    /// 64-bit floating-point value.
    Double,
    /// UTF-8 string value.
    #[default]
    String,
    /// Calendar date.
    Date,
    /// Local clock time.
    Time,
    /// Local date and time.
    Datetime,
    /// UTC instant.
    Instant,
    /// Timestamp value.
    Timestamp,
    /// Byte array.
    ByteArray,
    /// Class or type name.
    Class,
    /// Arbitrary-precision integer.
    BigInteger,
    /// Arbitrary-precision decimal.
    BigDecimal,
    /// String array.
    StringArray,
    /// Enumeration value.
    Enum,
    /// Enumeration array.
    EnumArray,
}

/// A named system setting containing zero, one, or multiple textual values.
#[derive(Clone, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    /// Stable setting name.
    pub name: String,
    /// Declared value type.
    #[serde(rename = "type")]
    pub data_type: DataType,
    /// Values represented in the source model's canonical string form.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[redact(skip)]
    pub values: Vec<String>,
    /// Whether callers may modify this setting.
    pub readonly: bool,
    /// Whether the setting may contain no values.
    pub nullable: bool,
    /// Whether the setting may contain more than one value.
    pub multiple: bool,
    /// Whether persisted values are encrypted.
    pub encrypted: bool,
    /// Optional human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC last-modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
}

impl Setting {
    /// Default read-only state.
    pub const DEFAULT_READONLY: bool = false;
    /// Default nullable state.
    pub const DEFAULT_NULLABLE: bool = true;
    /// Default multiple-value state.
    pub const DEFAULT_MULTIPLE: bool = true;
    /// Default encrypted state.
    pub const DEFAULT_ENCRYPTED: bool = false;
    /// Source representation of a null value.
    pub const NULL_STRING: &'static str = "\\CDATA\\[null\\CDATA\\]";
    /// Separator used for multiple string values.
    pub const STRING_DELIMITER: &'static str = "§\u{200B}§§";
    /// Separator used for multiple non-string values.
    pub const STANDARD_DELIMITER: &'static str = ",";

    /// Creates an empty setting with the source defaults.
    #[must_use]
    pub fn new(name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            name: name.into(),
            data_type,
            values: Vec::new(),
            readonly: Self::DEFAULT_READONLY,
            nullable: Self::DEFAULT_NULLABLE,
            multiple: Self::DEFAULT_MULTIPLE,
            encrypted: Self::DEFAULT_ENCRYPTED,
            description: None,
            create_time: None,
            modify_time: None,
        }
    }

    /// Returns whether the nullability and cardinality constraints are met.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        (self.nullable || !self.values.is_empty()) && (self.multiple || self.values.len() <= 1)
    }

    /// Encodes all values into the source database representation.
    #[must_use]
    pub fn persistent_value(&self) -> Option<String> {
        match self.values.as_slice() {
            [] => None,
            [value] => Some(value.clone()),
            values => {
                let delimiter = if self.data_type == DataType::String {
                    Self::STRING_DELIMITER
                } else {
                    Self::STANDARD_DELIMITER
                };
                Some(values.join(delimiter))
            }
        }
    }

    /// Replaces all values from the source database representation.
    pub fn set_persistent_value(&mut self, persistent_value: Option<&str>) {
        let Some(value) = persistent_value else {
            self.values.clear();
            return;
        };
        let delimiter = if self.data_type == DataType::String {
            Self::STRING_DELIMITER
        } else {
            Self::STANDARD_DELIMITER
        };
        self.values = if value.contains(delimiter) {
            value.split(delimiter).map(str::to_owned).collect()
        } else {
            vec![value.to_owned()]
        };
    }
}

impl Default for Setting {
    fn default() -> Self {
        Self::new(String::new(), DataType::default())
    }
}

impl Ord for Setting {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Setting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Namespace for predefined setting names.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
pub struct SettingName;

impl SettingName {
    /// Whether audit logs contain request bodies.
    pub const AUDIT_LOG_REQUEST_BODY_ENABLED: &'static str = "audit.log.request_body.enabled";
    /// Whether audit logs contain response bodies.
    pub const AUDIT_LOG_RESPONSE_BODY_ENABLED: &'static str = "audit.log.response_body.enabled";
    /// Audit-log hashing algorithm.
    pub const AUDIT_LOG_HASH_ALGORITHM: &'static str = "audit.log.hash.algorithm";
    /// Maximum task-shutdown wait in seconds.
    pub const TASK_SHUTDOWN_MAX_AWAIT_SECONDS: &'static str = "task.shutdown.max_await_seconds";
    /// Maximum token-generation retries.
    pub const TOKEN_GENERATOR_MAX_RETRIES: &'static str = "generator.token.retries.max";
    /// Login-session lifetime in seconds.
    pub const LOGIN_SESSION_MAX_AGE: &'static str = "login.session.timeout";
    /// Maximum consecutive login failures.
    pub const LOGIN_FAILURES_MAX_COUNT: &'static str = "login.failures.max_count";
    /// Login-failure record lifetime in seconds.
    pub const LOGIN_FAILURES_MAX_AGE: &'static str = "login.failures.max_age";
    /// Whether each user is restricted to one login session.
    pub const LOGIN_SINGLE_SESSION: &'static str = "login.single_session";
    /// Mobile verification-code lifetime in seconds.
    pub const VERIFY_MOBILE_MAX_AGE: &'static str = "verify.mobile.timeout";
    /// Mobile verification-code rate-limit count.
    pub const VERIFY_MOBILE_LIMIT_COUNT: &'static str = "verify.mobile.limit.count";
    /// Mobile verification-code rate-limit duration in seconds.
    pub const VERIFY_MOBILE_LIMIT_DURATION: &'static str = "verify.mobile.limit.duration";
    /// Minimum username length.
    pub const USER_USERNAME_LENGTH_MIN: &'static str = "user.username.length.min";
    /// Maximum username length.
    pub const USER_USERNAME_LENGTH_MAX: &'static str = "user.username.length.max";
    /// Username requirement description.
    pub const USER_USERNAME_REQUIREMENT: &'static str = "user.username.requirement";
    /// Username validation expression.
    pub const USER_USERNAME_REGEX: &'static str = "user.username.regex";
    /// Minimum password length.
    pub const USER_PASSWORD_LENGTH_MIN: &'static str = "user.password.length.min";
    /// Maximum password length.
    pub const USER_PASSWORD_LENGTH_MAX: &'static str = "user.password.length.max";
    /// Password requirement description.
    pub const USER_PASSWORD_REQUIREMENT: &'static str = "user.password.requirement";
    /// Password validation expression.
    pub const USER_PASSWORD_REGEX: &'static str = "user.password.regex";
    /// Email validation expression.
    pub const USER_EMAIL_REGEX: &'static str = "user.email.regex";
    /// Landline country-area validation expression.
    pub const USER_PHONE_COUNTRY_AREA_REGEX: &'static str = "user.phone.country_area.regex";
    /// Landline city-area validation expression.
    pub const USER_PHONE_CITY_AREA_REGEX: &'static str = "user.phone.city_area.regex";
    /// Landline number validation expression.
    pub const USER_PHONE_NUMBER_REGEX: &'static str = "user.phone.number.regex";
    /// Mobile country-area validation expression.
    pub const USER_MOBILE_COUNTRY_AREA_REGEX: &'static str = "user.mobile.country_area.regex";
    /// Mobile city-area validation expression.
    pub const USER_MOBILE_CITY_AREA_REGEX: &'static str = "user.mobile.city_area.regex";
    /// Mobile number validation expression.
    pub const USER_MOBILE_NUMBER_REGEX: &'static str = "user.mobile.number.regex";
}
