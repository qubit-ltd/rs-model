// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Setting values and stable setting names.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;


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
