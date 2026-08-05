use chrono::Utc;
use qubit_model::{
    commons::State,
    mixin::StatefulInfo,
    privilege::{Privileges, PrivilegesCodecError, Role},
};

fn role(privileges: &[&str]) -> Role {
    Role {
        id: Some(7),
        app: StatefulInfo::default(),
        code: "administrator".to_owned(),
        name: "Administrator".to_owned(),
        description: None,
        guest: Some(false),
        basic: Some(true),
        privileges: privileges.iter().map(|value| (*value).to_owned()).collect(),
        state: State::Normal,
        create_time: Utc::now(),
        modify_time: None,
        delete_time: None,
    }
}

#[test]
fn test_privileges_decode_strips_empty_segments() {
    let privileges = Privileges::decode(Some(" read , ,write,"))
        .expect("the Java-compatible codec should parse valid privileges")
        .expect("a non-null source should produce a collection");

    assert_eq!(privileges.0, ["read", "write"]);
    assert_eq!(
        privileges.encode().expect("privileges should encode"),
        "read,write"
    );
}

#[test]
fn test_privileges_encode_rejects_ambiguous_member() {
    let error = Privileges(vec!["read,write".to_owned()])
        .encode()
        .expect_err("a comma would lose collection boundaries");

    assert_eq!(error, PrivilegesCodecError::ContainsSeparator { index: 0 });
}

#[test]
fn test_role_collect_privileges_preserves_first_occurrence() {
    let roles = [role(&["read", "write"]), role(&["write", "admin"])];

    assert_eq!(Role::collect_privileges(&roles), ["read", "write", "admin"]);
}
