// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration tests for migrated person-domain classifications.

use chrono::Utc;

use qubit_mixin::Normalizable;
use qubit_model::commons::AuthorizeRecord;
use qubit_model::commons::State;
use qubit_model::commons::VerifyState;
use qubit_model::contact::Phone;
use qubit_model::controller::RegisterUserParams;
use qubit_model::medical::MedicareType;
use qubit_model::mixin::StatefulInfo;
use qubit_model::person::Gender;
use qubit_model::person::Person;
use qubit_model::person::PersonInfo;
use qubit_model::person::SocialNetwork;
use qubit_model::person::SocialNetworkAccount;
use qubit_model::person::User;
use qubit_redact::Redact as _;

#[test]
fn test_person_classification_enums_preserve_source_variants() {
    assert_eq!(Gender::Female, Gender::Female);
    assert_eq!(
        MedicareType::NewRuralCooperative,
        MedicareType::NewRuralCooperative
    );
}

#[test]
fn test_user_redacts_the_source_password_field() {
    let user = User {
        id: Some(7),
        username: "ada".to_owned(),
        password: "raw-password".to_owned(),
        name: None,
        nickname: None,
        gender: None,
        mobile: Some(Phone {
            country_area: Some("86".to_owned()),
            city_area: None,
            number: "13900000000".to_owned(),
        }),
        mobile_verified: Some(VerifyState::Valid),
        email: Some("ada@example.test".to_owned()),
        email_verified: Some(VerifyState::Valid),
        avatar: None,
        url: None,
        description: None,
        organization: None,
        state: State::Normal,
        last_login: AuthorizeRecord::default(),
        change_password: false,
        valid_time: None,
        expired_time: None,
        comment: None,
        predefined: false,
        test: false,
        create_time: Utc::now(),
        modify_time: None,
        delete_time: None,
    };

    let redacted = format!("{:?}", user.redacted());

    assert!(!redacted.contains("raw-password"));
    assert!(redacted.contains("ada"));
    let _: Option<StatefulInfo> = user.organization;
}

#[test]
fn test_person_compact_info_updates_and_checks_benefit_coverage() {
    let mut person = Person::default();
    person.set_info(&PersonInfo {
        id: Some(7),
        name: "Ada".into(),
        mobile: Some(Phone::from("13800138000")),
        email: Some("ada@example.test".into()),
        ..PersonInfo::default()
    });
    assert_eq!(person.id, Some(7));
    assert_eq!(person.info().email.as_deref(), Some("ada@example.test"));

    person.has_medicare = Some(false);
    person.has_social_security = Some(false);
    assert!(!person.has_medicare_or_social_security());
    person.has_social_security = Some(true);
    assert!(person.has_medicare_or_social_security());
    person.has_medicare = Some(true);
    assert!(person.has_medicare_or_social_security());
}

#[test]
fn test_social_network_account_uses_registration_defaults_and_normalizes_text()
{
    let default_account = SocialNetworkAccount::from_register_params(
        &RegisterUserParams::default(),
    );
    assert_eq!(default_account.social_network, SocialNetwork::Wechat);
    assert_eq!(default_account.app_id, "");
    assert_eq!(default_account.open_id, "");

    let params = RegisterUserParams {
        username: "  alice  ".into(),
        social_network: Some(SocialNetwork::Zhihu),
        app_id: Some(" app ".into()),
        open_id: Some(" open ".into()),
        nickname: Some(" nick ".into()),
        avatar: Some(" avatar ".into()),
        ..RegisterUserParams::default()
    };
    let mut account = SocialNetworkAccount::from_register_params(&params);
    account.normalize();
    assert_eq!(account.username, "alice");
    assert_eq!(account.social_network, SocialNetwork::Zhihu);
    assert_eq!(account.app_id, "app");
    assert_eq!(account.open_id, "open");
    assert_eq!(account.nickname.as_deref(), Some("nick"));
    assert_eq!(account.avatar.as_deref(), Some("avatar"));
}
