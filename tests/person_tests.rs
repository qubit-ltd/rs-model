//! Integration tests for migrated person-domain classifications.

use chrono::Utc;
use qubit_model::{
    commons::{AuthorizeRecord, State, VerifyState},
    contact::Phone,
    medical::MedicareType,
    mixin::StatefulInfo,
    person::{Gender, User},
};
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
