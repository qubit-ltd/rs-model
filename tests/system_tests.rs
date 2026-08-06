// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::{TimeZone, Utc};
use qubit_mixin::{Emptyful, Normalizable};
use qubit_model::{
    commons::{State, Token},
    contact::Location,
    mixin::StatefulInfo,
    person::UserInfo,
    privilege::Role,
    system::{
        Action, Environment, ErrorInfo, Expired, ExpiredReason, Host, Log,
        LogicRelation, OperationLog, OperationLogInfo, Platform, Session,
        Setting, VerifyCode, VerifyScene,
    },
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;
use serde::Serialize;
use std::io;

fn assert_redact<T: Redact>() {}

/// A deterministic writer that fails on the selected write operation.
struct FailingWriter {
    failure_at: usize,
    writes: usize,
}

impl FailingWriter {
    /// Creates a writer that fails exactly at `failure_at`.
    const fn new(failure_at: usize) -> Self {
        Self {
            failure_at,
            writes: 0,
        }
    }
}

impl io::Write for FailingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.writes += 1;
        if self.writes == self.failure_at {
            Err(io::Error::other("intentional test writer failure"))
        } else {
            Ok(buffer.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Verifies that every serializer write boundary propagates its I/O error.
fn assert_serializer_propagates_each_write_error<T: Serialize>(value: &T) {
    for failure_at in 1..=4_096 {
        let mut serializer =
            serde_json::Serializer::new(FailingWriter::new(failure_at));
        if value.serialize(&mut serializer).is_ok() {
            return;
        }
    }
    panic!("serializer did not complete within the expected write boundary");
}

/// Serializes a value through the public JSON text representation.
fn json_value<T: Serialize>(value: &T) -> serde_json::Value {
    let text = serde_json::to_string(value)
        .expect("value should serialize to JSON text");
    serde_json::from_str(&text).expect("JSON text should parse into a value")
}

#[test]
fn system_public_models_preserve_source_shapes_and_traits() {
    assert_redact::<Environment>();
    assert_redact::<Expired>();
    assert_redact::<Host>();
    assert_redact::<Log>();
    assert_redact::<OperationLog>();
    assert_redact::<OperationLogInfo>();
    assert_redact::<Session>();
    assert_redact::<Setting>();
    assert_redact::<VerifyCode>();

    assert_eq!(metadata_of::<Environment>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<Expired>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<Host>().struct_fields().len(), 3);
    assert_eq!(metadata_of::<Log>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<OperationLog>().struct_fields().len(), 51);
    assert_eq!(metadata_of::<OperationLogInfo>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<Session>().struct_fields().len(), 11);
}

#[test]
fn system_model_metadata_preserves_source_constraints() {
    let environment = metadata_of::<Environment>();
    for field in ["ip", "location", "platform", "udid", "push_token"] {
        assert!(environment.indexes().any(|index| index.contains(field)));
    }

    let host = metadata_of::<Host>();
    assert_eq!(host.primary_key().unwrap().fields()[0].name(), "id");
    assert_eq!(
        host.unique_constraints()
            .next()
            .unwrap()
            .comparison_of("udid"),
        Some(qubit_model_metadata::UniqueComparison::Exact)
    );

    let operation = metadata_of::<OperationLog>();
    assert_eq!(operation.primary_key().unwrap().fields()[0].name(), "id");
    for field in [
        "action",
        "request_time",
        "client_ip",
        "request_id",
        "service",
    ] {
        assert!(operation.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn system_enums_and_session_helpers_preserve_source_behavior() {
    assert_eq!(ExpiredReason::SingleSession.id(), "single_session");
    assert_eq!(Action::PurgeAll.display_name(), "Purge all");
    assert_eq!(Action::from_name("BATCH_UPDATE"), Some(Action::BatchUpdate));
    assert_eq!(Action::from_name("batch_update"), None);
    assert_eq!(
        serde_json::to_string(&Platform::IpadOs).unwrap(),
        "\"IPAD_OS\""
    );
    assert_eq!(
        serde_json::to_string(&VerifyScene::Login).unwrap(),
        "\"LOGIN\""
    );

    let session = Session {
        roles: vec!["ADMIN".into()],
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        ..Session::default()
    };
    assert!(session.has_role("ADMIN"));
    assert_eq!(session.username(), Some("alice"));
    assert_eq!(json_value(&session)["username"], "alice");

    Session::reset();
    Session::set_current_user(session.user.clone());
    assert_eq!(
        Session::current_user().map(|user| user.username),
        Some("alice".into())
    );
    Session::reset();
}

#[test]
fn session_thread_local_accessors_normalize_values_and_serialize_present_fields()
 {
    Session::reset();
    assert_eq!(Session::current_session(), Session::default());
    let app = StatefulInfo {
        id: Some(1),
        code: "APP".into(),
        name: "Application".into(),
        ..StatefulInfo::default()
    };
    let user = UserInfo {
        username: "alice".into(),
        ..UserInfo::default()
    };
    let token = Token {
        value: "token".into(),
        ..Token::default()
    };
    Session::set_current_session(Session::default());
    Session::set_current_app(None);
    Session::set_current_user(None);
    Session::set_current_user_token(None);
    assert_eq!(Session::current_app(), None);
    assert_eq!(Session::current_user(), None);
    assert_eq!(Session::current_user_token(), None);
    Session::set_current_app(Some(app.clone()));
    Session::set_current_user(Some(user.clone()));
    Session::set_current_user_token(Some(token.clone()));
    assert_eq!(Session::current_app(), Some(app.clone()));
    assert_eq!(Session::current_user(), Some(user.clone()));
    assert_eq!(Session::current_user_token(), Some(token.clone()));
    let count = Session::with_current_session(|session| {
        session.roles = vec![" ADMIN ".into(), "USER".into()];
        session.privileges = vec![" READ ".into()];
        session.set_roles_and_privileges(&[Role {
            id: None,
            app: StatefulInfo::default(),
            code: "ADMIN".into(),
            name: "Administrator".into(),
            description: None,
            guest: None,
            basic: None,
            privileges: vec!["READ".into(), "WRITE".into()],
            state: State::Normal,
            create_time: Utc::now(),
            modify_time: None,
            delete_time: None,
        }]);
        session.normalize();
        session.roles.len()
    });
    assert_eq!(count, 1);

    let session = Session {
        id: Some(1),
        app: Some(app),
        user: Some(user),
        organization: Some(StatefulInfo::default()),
        token: Some(token),
        roles: vec!["ADMIN".into()],
        privileges: vec!["READ".into()],
        environment: Some(Environment::default()),
        last_active_time: Some(Utc::now()),
        expired: Some(Expired::default()),
        create_time: Some(Utc::now()),
    };
    let json = json_value(&session);
    for field in [
        "id",
        "app",
        "user",
        "organization",
        "token",
        "roles",
        "privileges",
        "environment",
        "last_active_time",
        "expired",
        "create_time",
        "username",
    ] {
        assert!(json.get(field).is_some(), "{field} must be serialized");
    }
    assert_eq!(json_value(&Session::default()), serde_json::json!({}));
    assert!(!Session::default().has_role("ADMIN"));
    assert_eq!(Session::default().username(), None);

    Session::set_super_admin_session(Some(session));
    assert!(Session::is_super_admin_mode());
    assert!(Session::super_admin_session().is_some());
    Session::clear_super_admin_session();
    assert!(!Session::is_super_admin_mode());
    Session::reset();
}

#[test]
fn session_serialization_propagates_every_writer_failure() {
    let session = Session {
        id: Some(1),
        app: Some(StatefulInfo {
            id: Some(2),
            ..StatefulInfo::default()
        }),
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        organization: Some(StatefulInfo {
            id: Some(3),
            ..StatefulInfo::default()
        }),
        token: Some(Token {
            value: "token".into(),
            ..Token::default()
        }),
        roles: vec!["ADMIN".into()],
        privileges: vec!["READ".into()],
        environment: Some(Environment::default()),
        last_active_time: Some(Utc::now()),
        expired: Some(Expired::default()),
        create_time: Some(Utc::now()),
    };

    assert_serializer_propagates_each_write_error(&session);
}

#[test]
fn system_normalization_preserves_source_empty_and_text_behavior() {
    let mut environment = Environment {
        ip: Some("  ".into()),
        udid: Some(" device ".into()),
        ..Environment::default()
    };
    environment.normalize();
    assert_eq!(environment.ip, None);
    assert_eq!(environment.udid.as_deref(), Some("device"));
    assert!(!environment.is_empty());

    let locations = [
        Environment {
            location: Some(Location {
                longitude: 0.into(),
                latitude: 0.into(),
                altitude: None,
                coordinate_system: None,
            }),
            ..Environment::default()
        },
        Environment {
            platform: Some(Platform::Android),
            ..Environment::default()
        },
        Environment {
            push_token: Some("push".into()),
            ..Environment::default()
        },
    ];
    assert!(Environment::default().is_empty());
    assert!(Emptyful::is_empty(&Environment::default()));
    for environment in locations {
        assert!(!environment.is_empty());
        assert!(!environment.is_normalized_empty());
    }
}

#[test]
fn operation_log_projects_and_assigns_compact_info() {
    let request_time = Utc.timestamp_opt(1_700_000_000, 0).unwrap();
    let mut log = OperationLog {
        id: Some(5),
        action: Action::Update,
        resource: Some("USER".into()),
        property: Some("status".into()),
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        app: Some(StatefulInfo {
            name: "portal".into(),
            ..StatefulInfo::default()
        }),
        client_ip: "127.0.0.1".into(),
        success: Some(false),
        error: Some(ErrorInfo {
            code: "DENIED".into(),
            message: Some("denied".into()),
            ..ErrorInfo::default()
        }),
        request_time: Some(request_time),
        ..OperationLog::default()
    };
    let info = log.info();
    assert_eq!(info.username.as_deref(), Some("alice"));
    assert_eq!(info.property.as_deref(), Some("status"));
    assert_eq!(info.app.as_deref(), Some("portal"));
    assert_eq!(info.error_code.as_deref(), Some("DENIED"));
    assert_eq!(info.error_message.as_deref(), Some("denied"));
    assert_eq!(info.timestamp, Some(request_time));

    let replacement = OperationLogInfo {
        action: Action::Get,
        username: Some("bob".into()),
        app: Some("admin".into()),
        error_code: Some("DENIED".into()),
        ..OperationLogInfo::default()
    };
    log.assign_info(&replacement);
    assert_eq!(log.action, Action::Get);
    assert_eq!(
        log.user.as_ref().map(|user| user.username.as_str()),
        Some("bob")
    );
    assert_eq!(
        log.error.as_ref().map(|error| error.code.as_str()),
        Some("DENIED")
    );
    assert_eq!(log.app.as_ref().map(|app| app.name.as_str()), Some("admin"));
}

#[test]
fn system_redaction_hides_tokens_request_material_and_device_ids() {
    let environment = Environment {
        udid: Some("private-device".into()),
        push_token: Some("private-push-token".into()),
        ..Environment::default()
    };
    let rendered = format!("{:?}", environment.redacted());
    assert!(!rendered.contains("private-device"));
    assert!(!rendered.contains("private-push-token"));

    let log = OperationLog {
        user_token_hash: Some("private-user-hash".into()),
        request_body: Some("private-request-body".into()),
        response_body: Some("private-response-body".into()),
        ..OperationLog::default()
    };
    let rendered = format!("{:?}", log.redacted());
    assert!(!rendered.contains("private-user-hash"));
    assert!(!rendered.contains("private-request-body"));
    assert!(!rendered.contains("private-response-body"));
}

#[test]
fn logs_order_by_source_timestamp() {
    let earlier = Log {
        timestamp: Utc.timestamp_opt(1, 0).single(),
        ..Log::default()
    };
    let later = Log {
        timestamp: Utc.timestamp_opt(2, 0).single(),
        ..Log::default()
    };
    assert!(earlier < later);

    let expired = Expired {
        time: Utc.timestamp_opt(3, 0).single(),
        reason: ExpiredReason::Timeout,
    };
    assert_eq!(expired.reason, ExpiredReason::Timeout);
}

#[test]
fn system_enum_mappings_cover_all_action_expiration_and_logic_variants() {
    for action in [
        Action::Login,
        Action::Logout,
        Action::List,
        Action::Get,
        Action::Add,
        Action::Update,
        Action::Delete,
        Action::Restore,
        Action::Purge,
        Action::PurgeAll,
        Action::Erase,
        Action::BatchAdd,
        Action::BatchUpdate,
        Action::BatchDelete,
        Action::BatchRestore,
        Action::BatchPurge,
        Action::BatchErase,
        Action::Clear,
        Action::Import,
        Action::Export,
        Action::TestExistence,
        Action::Bind,
        Action::Register,
        Action::Reset,
        Action::Check,
        Action::Unregister,
        Action::Unbound,
        Action::Send,
        Action::Authenticate,
        Action::Refresh,
        Action::Count,
        Action::ListAll,
        Action::ListFirst,
        Action::ForEach,
        Action::AddOrUpdate,
        Action::PerformAction,
    ] {
        assert!(!action.display_name().is_empty());
        let name = json_value(&action)
            .as_str()
            .expect("action wire value is textual")
            .to_owned();
        assert_eq!(Action::from_name(&name), Some(action));
    }
    assert_eq!(Action::default(), Action::Get);
    assert_eq!(Action::from_name("unknown"), None);
    assert_eq!(ExpiredReason::Logout.id(), "logout");
    assert_eq!(ExpiredReason::Timeout.id(), "timeout");
    assert_eq!(ExpiredReason::SingleSession.id(), "single_session");
    assert_eq!(ExpiredReason::Maintenance.id(), "maintenance");
    assert_eq!(ExpiredReason::None.id(), "none");
    assert_eq!(ExpiredReason::default(), ExpiredReason::None);
    assert_eq!(LogicRelation::And.symbol(), "AND");
    assert_eq!(LogicRelation::Or.symbol(), "OR");
    assert_eq!(LogicRelation::Not.symbol(), "NOT");
    assert_eq!(LogicRelation::default(), LogicRelation::And);
}
