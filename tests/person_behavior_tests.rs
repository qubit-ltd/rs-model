// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use qubit_model::commons::CredentialInfo;
use qubit_model::commons::CredentialType;
use qubit_model::commons::VerifyState;
use qubit_model::contact::Address;
use qubit_model::contact::Contact;
use qubit_model::contact::Phone;
use qubit_model::order::Buyer;
use qubit_model::order::Client;
use qubit_model::order::Consignee;
use qubit_model::person::Gender;
use qubit_model::person::Person;
use qubit_model::person::PersonInfo;
use qubit_model_metadata::metadata_of;

fn credential(id: Option<i64>) -> CredentialInfo {
    CredentialInfo {
        id,
        r#type: CredentialType::IdentityCard,
        number: "320101199001010011".into(),
        verified: Some(VerifyState::Valid),
    }
}

#[test]
fn person_projects_and_assigns_person_info_like_the_source() {
    let mut person = Person {
        id: Some(7),
        name: "Ada".into(),
        username: Some("ada".into()),
        gender: Some(Gender::Female),
        credential: Some(credential(Some(21))),
        contact: Contact::create(
            None,
            Some(Phone::from("13800138000")),
            Some("ada@example.test".into()),
            None,
            None,
        ),
        test: true,
        ..Person::default()
    };

    let info = person.info();
    assert_eq!(info.id, Some(7));
    assert_eq!(info.username.as_deref(), Some("ada"));
    assert_eq!(info.mobile.as_ref().unwrap().number, "13800138000");
    assert_eq!(info.email.as_deref(), Some("ada@example.test"));

    person.assign_info(&PersonInfo {
        id: Some(8),
        name: "Grace".into(),
        username: Some("ignored-by-source-assign".into()),
        gender: None,
        birthday: None,
        credential: None,
        mobile: None,
        email: None,
        photo: None,
        test: false,
        delete_time: None,
    });
    assert_eq!(person.id, Some(8));
    assert_eq!(person.name, "Grace");
    assert_eq!(person.username.as_deref(), Some("ada"));
    assert_eq!(person.contact, None);
}

#[test]
fn person_assigns_client_buyer_and_consignee_views() {
    let mut person = Person::default();
    let client = Client {
        id: Some(1),
        name: "Client".into(),
        credential: Some(credential(None)),
        gender: Some(Gender::Female),
        birthday: None,
        mobile: Some(Phone::from("13900000000")),
        email: Some("client@example.test".into()),
        has_medicare: Some(true),
        medicare_type: None,
        medicare_card: None,
        medicare_city: None,
        has_social_security: Some(false),
        social_security_card: None,
        social_security_city: None,
        guardian: None,
        return_status: None,
        kinship: None,
        payload: None,
    };
    person.assign_client(&client);
    assert_eq!(person.id, Some(1));
    assert_eq!(person.contact.as_ref().unwrap().email, client.email);
    assert!(person.has_medicare_or_social_security());

    let buyer = Buyer {
        id: Some(2),
        user_id: Some(20),
        name: "Buyer".into(),
        credential: None,
        gender: None,
        birthday: None,
        mobile: None,
        email: None,
    };
    person.assign_buyer(&buyer);
    assert_eq!(person.id, Some(2));
    assert_eq!(person.name, "Buyer");

    let consignee = Consignee {
        id: Some(3),
        user_id: Some(30),
        title: None,
        name: "Consignee".into(),
        mobile: Phone::from("13700000000"),
        email: None,
        credential: None,
        address: Address::default(),
        comment: String::new(),
        create_time: Default::default(),
        modify_time: None,
        delete_time: None,
    };
    person.assign_consignee(&consignee);
    assert_eq!(person.id, Some(3));
    assert_eq!(
        person
            .contact
            .as_ref()
            .unwrap()
            .mobile
            .as_ref()
            .unwrap()
            .number,
        "13700000000"
    );
    assert!(person.contact.as_ref().unwrap().address.is_some());
}

#[test]
fn person_identity_prefers_ids_then_falls_back_to_credential() {
    let identity = credential(Some(1));
    let person = Person {
        id: Some(7),
        credential: Some(identity.clone()),
        ..Person::default()
    };
    let same_id = Buyer {
        id: Some(7),
        credential: None,
        ..Buyer::default()
    };
    let different_id_same_credential = Buyer {
        id: Some(8),
        credential: Some(identity.clone()),
        ..Buyer::default()
    };
    assert!(person.is_same(&same_id));
    assert!(!person.is_same(&different_id_same_credential));

    let person_without_id = Person {
        credential: Some(identity),
        ..Person::default()
    };
    let same_credential = PersonInfo {
        credential: Some(credential(None)),
        ..PersonInfo::default()
    };
    assert!(person_without_id.is_same(&same_credential));
}

#[test]
fn person_model_metadata_preserves_source_uniques_indexes_and_references() {
    let metadata = metadata_of::<Person>();
    for field in [
        "source",
        "category",
        "name",
        "username",
        "gender",
        "birthday",
        "birth_time",
        "birth_country",
        "birth_province",
        "birth_city",
        "has_medicare",
        "medicare_type",
        "medicare_city",
        "has_social_security",
        "social_security_city",
        "contact",
        "has_child",
        "test",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(
            metadata.indexes().any(|index| index.contains(field)),
            "missing index for {field}"
        );
    }
    for field in ["username", "credential"] {
        assert!(
            metadata
                .unique_constraints()
                .any(|unique| unique.contains(field)),
            "missing unique constraint for {field}"
        );
    }
    for field in [
        "source",
        "category",
        "birth_country",
        "birth_province",
        "birth_city",
        "credential",
        "guardian",
        "photo",
    ] {
        assert!(
            metadata.field(field).unwrap().reference().is_some(),
            "missing reference for {field}"
        );
    }
}
