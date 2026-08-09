// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Detailed HTTP operation audit logs.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Action;
use super::ErrorInfo;
use super::LogicRelation;
use super::OperationLogInfo;
use crate::commons::App;
use crate::mixin::StatefulInfo;
use crate::person::User;
use crate::person::UserInfo;

/// Represents a complete audited request, response, caller, service, and trace record.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct OperationLog {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Audited action.
    #[model(index)]
    pub action: Action,

    /// Optional resource name.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub resource: Option<String>,

    /// Optional resource property.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub property: Option<String>,

    /// Required permission names.
    #[model(index, sequence(min_items = 1))]
    pub permissions: Vec<String>,

    /// Relation joining permissions.
    #[model(index)]
    pub permission_logic: LogicRelation,

    /// Optional encoded permission selectors.
    #[model(index)]
    pub selectors: Option<String>,

    /// Optional user information.
    #[model(index, reference(target = User, target_field = info, must_exist = true), opaque)]
    #[redact(nested)]
    pub user: Option<UserInfo>,

    /// Optional hash of the user token.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub user_token_hash: Option<String>,

    /// Optional application information.
    #[model(index, reference(target = App, target_field = info, must_exist = true), opaque)]
    pub app: Option<StatefulInfo>,

    /// Optional hash of the application token.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub app_token_hash: Option<String>,

    /// Optional operation outcome.
    #[model(index)]
    pub success: Option<bool>,

    /// Optional structured error.
    #[model(index)]
    #[redact(nested)]
    pub error: Option<ErrorInfo>,

    /// UTC request timestamp.
    #[model(index, time(precision = millisecond, normalization = utc))]
    pub request_time: Option<DateTime<Utc>>,

    /// Optional UTC response timestamp.
    #[model(index, time(precision = millisecond, normalization = utc))]
    pub response_time: Option<DateTime<Utc>>,

    /// Optional latency in milliseconds.
    #[model(index)]
    pub latency: Option<i64>,

    /// Client IP address.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub client_ip: String,

    /// Optional HTTP user-agent value.
    #[redact(level = "secret")]
    pub user_agent: Option<String>,

    /// Optional HTTP referer value.
    #[redact(level = "secret")]
    pub referer: Option<String>,

    /// Request host.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub request_host: String,

    /// HTTP method.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub http_method: String,

    /// Complete request URI.
    #[redact(level = "secret")]
    pub request_uri: String,

    /// Request path.
    pub request_path: String,

    /// Optional resolved request path.
    pub resolved_request_path: Option<String>,

    /// Optional request headers.
    #[redact(level = "secret")]
    pub request_header: Option<String>,

    /// Optional request-header hash.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub request_header_hash: Option<String>,

    /// Optional request query.
    #[redact(level = "secret")]
    pub request_query: Option<String>,

    /// Optional request-query hash.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub request_query_hash: Option<String>,

    /// Optional request body.
    #[redact(level = "secret")]
    pub request_body: Option<String>,

    /// Optional request-body hash.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub request_body_hash: Option<String>,

    /// Request-body size in bytes.
    pub request_body_size: i64,

    /// Optional response headers.
    #[redact(level = "secret")]
    pub response_header: Option<String>,

    /// Optional response-header hash.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub response_header_hash: Option<String>,

    /// Optional HTTP response code.
    #[model(index)]
    pub response_code: Option<i32>,

    /// Optional response body.
    #[redact(level = "secret")]
    pub response_body: Option<String>,

    /// Optional response-body hash.
    #[model(text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    #[redact(level = "secret")]
    pub response_body_hash: Option<String>,

    /// Optional response-body size in bytes.
    pub response_body_size: Option<i64>,

    /// Optional trace identifier.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub trace_id: Option<String>,

    /// Optional span identifier.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub span_id: Option<String>,

    /// Optional correlation identifier.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub correlation_id: Option<String>,

    /// Optional request identifier.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub request_id: Option<String>,

    /// Optional API version.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub api_version: Option<String>,

    /// Optional remote endpoint.
    #[model(index, text(min_chars = 1, max_chars = 2048, repertoire = ascii))]
    pub endpoint: Option<String>,

    /// Optional service name.
    #[model(index, text(min_chars = 1, max_chars = 256))]
    pub service: Option<String>,

    /// Optional service host.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub service_host: Option<String>,

    /// Optional worker thread name.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub thread: Option<String>,

    /// Optional service instance name.
    #[model(index, text(min_chars = 1, max_chars = 256, repertoire = ascii))]
    pub instance: Option<String>,

    /// Hash algorithm used for stored request and response hashes.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub hash_algorithm: String,

    /// Optional encoded list of sensitive fields.
    #[redact(level = "secret")]
    pub sensitive_fields: Option<String>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}

impl OperationLog {
    /// Projects this log into its compact information form.
    #[must_use]
    pub fn info(&self) -> OperationLogInfo {
        OperationLogInfo {
            id: self.id,
            action: self.action,
            resource: self.resource.clone(),
            property: self.property.clone(),
            username: self.user.as_ref().map(|user| user.username.clone()),
            app: self.app.as_ref().map(|app| app.name.clone()),
            client_ip: self.client_ip.clone(),
            success: self.success,
            error_code: self.error.as_ref().map(|error| error.code.clone()),
            error_message: self.error.as_ref().and_then(|error| error.message.clone()),
            timestamp: self.request_time,
        }
    }

    /// Assigns the fields carried by a compact operation-log projection.
    pub fn assign_info(&mut self, info: &OperationLogInfo) {
        self.id = info.id;
        self.action = info.action;
        self.resource.clone_from(&info.resource);
        self.property.clone_from(&info.property);
        self.user = info.username.as_ref().map(|username| UserInfo {
            username: username.clone(),
            ..UserInfo::default()
        });
        self.app = info.app.as_ref().map(|name| StatefulInfo {
            name: name.clone(),
            ..StatefulInfo::default()
        });
        self.client_ip.clone_from(&info.client_ip);
        self.success = info.success;
        self.error = info.error_code.as_ref().map(|code| ErrorInfo {
            code: code.clone(),
            message: info.error_message.clone(),
            ..ErrorInfo::default()
        });
        self.request_time = info.timestamp;
    }
}
