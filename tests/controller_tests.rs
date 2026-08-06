// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::{
    controller::{
        AppAuthenticateParams, AuditableQueryParams, BindDeviceParams, BindEmployeeParams,
        BindPersonParams, LoginParams, LoginResponse, NullSortOption, RegisterUserParams,
        SortOrder, UnupdatableQueryParams, UpdatePasswordParams,
    },
    person::{SocialNetwork, SocialNetworkAccount, User, UserInfo},
    system::Session,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires diagnostic redaction for one controller model.
fn assert_redact<T: Redact>() {}

#[test]
fn test_controller_models_preserve_source_shapes_and_traits() {
    assert_redact::<AppAuthenticateParams>();
    assert_redact::<AuditableQueryParams>();
    assert_redact::<BindDeviceParams>();
    assert_redact::<BindEmployeeParams>();
    assert_redact::<BindPersonParams>();
    assert_redact::<LoginParams>();
    assert_redact::<LoginResponse>();
    assert_redact::<RegisterUserParams>();
    assert_redact::<UnupdatableQueryParams>();
    assert_redact::<UpdatePasswordParams>();

    assert_eq!(
        metadata_of::<AppAuthenticateParams>().struct_fields().len(),
        6
    );
    assert_eq!(
        metadata_of::<AuditableQueryParams>().struct_fields().len(),
        13
    );
    assert_eq!(metadata_of::<BindDeviceParams>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<BindEmployeeParams>().struct_fields().len(), 6);
    assert_eq!(metadata_of::<BindPersonParams>().struct_fields().len(), 6);
    assert_eq!(metadata_of::<LoginParams>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<LoginResponse>().struct_fields().len(), 5);
    assert_eq!(
        metadata_of::<RegisterUserParams>().struct_fields().len(),
        14
    );
    assert_eq!(
        metadata_of::<UnupdatableQueryParams>()
            .struct_fields()
            .len(),
        11
    );
    assert_eq!(
        metadata_of::<UpdatePasswordParams>().struct_fields().len(),
        4
    );
}

#[test]
fn test_query_params_preserve_source_defaults_and_null_ordering() {
    assert_eq!(AuditableQueryParams::default().page_index, Some(0));
    assert_eq!(UnupdatableQueryParams::default().page_index, Some(0));
    assert_eq!(
        NullSortOption::NullSmallest.compare_none(true, false, SortOrder::Desc),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn test_controller_authentication_material_is_redacted() {
    let login = LoginParams {
        password: Some("private-password".into()),
        open_id: Some("private-open-id".into()),
        ..LoginParams::default()
    };
    let rendered = format!("{:?}", login.redacted());
    assert!(!rendered.contains("private-password"));
    assert!(!rendered.contains("private-open-id"));

    let update = UpdatePasswordParams {
        old_password: Some("old-secret".into()),
        new_password: "new-secret".into(),
        ..UpdatePasswordParams::default()
    };
    let rendered = format!("{:?}", update.redacted());
    assert!(!rendered.contains("old-secret"));
    assert!(!rendered.contains("new-secret"));

    let mut desensitized = LoginParams::default();
    desensitized.desensitize();
    assert_eq!(desensitized.password.as_deref(), Some("--------"));
    assert_eq!(desensitized.open_id.as_deref(), Some("--------"));
}

#[test]
fn test_register_login_response_and_social_account_preserve_source_projections() {
    let register = RegisterUserParams {
        username: "alice".into(),
        social_network: Some(SocialNetwork::Wechat),
        app_id: Some("app".into()),
        open_id: Some("openid".into()),
        ..RegisterUserParams::default()
    };
    let account = SocialNetworkAccount::from_register_params(&register);
    assert_eq!(account.username, "alice");
    assert_eq!(account.open_id, "openid");

    let session = Session {
        user: Some(UserInfo {
            username: "alice".into(),
            ..UserInfo::default()
        }),
        roles: vec!["ADMIN".into()],
        ..Session::default()
    };
    let response = LoginResponse::from_session(&session);
    assert_eq!(
        response.user.as_ref().map(|user| user.username.as_str()),
        Some("alice")
    );
    assert_eq!(response.roles, vec!["ADMIN"]);
}

#[test]
fn test_register_user_params_projects_users_and_desensitizes_credentials() {
    let user = User {
        id: None,
        username: "alice".into(),
        password: "secret".into(),
        name: Some("Alice".into()),
        nickname: Some("ali".into()),
        gender: None,
        mobile: None,
        mobile_verified: None,
        email: None,
        email_verified: None,
        avatar: None,
        url: None,
        description: None,
        organization: None,
        state: qubit_model::commons::State::Normal,
        last_login: qubit_model::commons::AuthorizeRecord::default(),
        change_password: false,
        valid_time: None,
        expired_time: None,
        comment: None,
        predefined: false,
        test: false,
        create_time: chrono::Utc::now(),
        modify_time: None,
        delete_time: None,
    };
    let mut params = RegisterUserParams::from_user(&user);
    assert_eq!(params.username, "alice");
    assert_eq!(params.password, "secret");
    let info = UserInfo {
        username: "bob".into(),
        ..UserInfo::default()
    };
    assert_eq!(RegisterUserParams::from_user_info(&info).username, "bob");
    params.desensitize();
    assert_eq!(params.password, "--------");
    assert_eq!(params.open_id.as_deref(), Some("--------"));
}
