// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use bigdecimal::BigDecimal;
use chrono::Utc;
use qubit_mixin::{Info, Normalizable};
use qubit_model::{
    activity::{Activity, ActivityCoupon, ActivityProductItem},
    commons::{CredentialInfo, CredentialType, Currency, State, VerifyState},
    contact::{Address, Contact, Phone},
    mixin::StatefulInfo,
    order::{Buyer, OrderInfo, OrderStatus, PayType},
    person::Person,
    product::Seller,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_activity_models_preserve_all_java_fields() {
    assert_eq!(metadata_of::<Activity>().struct_fields().len(), 12);
    assert_eq!(
        metadata_of::<ActivityProductItem>().struct_fields().len(),
        5
    );
    assert_eq!(metadata_of::<ActivityCoupon>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Person>().struct_fields().len(), 43);
    assert_eq!(metadata_of::<Contact>().struct_fields().len(), 9);

    assert_redact::<Activity>();
    assert_redact::<ActivityProductItem>();
    assert_redact::<ActivityCoupon>();
    assert_redact::<Person>();
    assert_redact::<Contact>();
}

#[test]
fn test_activity_model_metadata_preserves_source_constraints() {
    let activity = metadata_of::<Activity>();
    assert_eq!(
        activity
            .primary_key()
            .expect("activity primary key")
            .fields()[0]
            .name(),
        "id"
    );
    assert!(
        activity
            .unique_constraints()
            .any(|unique| unique.contains("code"))
    );

    let item = metadata_of::<ActivityProductItem>();
    assert!(item.primary_key().is_none());
    assert!(item.field("activity_id").unwrap().reference().is_some());
    assert!(item.field("product").unwrap().reference().is_some());
}

#[test]
fn test_activity_defaults_missing_state_to_normal() {
    let now = Utc::now();
    let activity = Activity {
        id: None,
        code: "summer".into(),
        name: "Summer".into(),
        app: Info::default(),
        items: vec![],
        description: None,
        start_time: now,
        end_time: now,
        state: State::Disabled,
        create_time: now,
        modify_time: None,
        delete_time: None,
    };
    let mut json = serde_json::to_value(activity).unwrap();
    json.as_object_mut().unwrap().remove("state");

    let decoded: Activity = serde_json::from_value(json).unwrap();
    assert_eq!(decoded.state, State::Normal);
}

#[test]
fn test_contact_create_and_verify_state_match_java_behavior() {
    assert_eq!(Contact::create(None, None, None, None, None), None);

    let mut contact = Contact::create(
        None,
        Some(Phone {
            country_area: Some("86".to_owned()),
            city_area: None,
            number: "13800138000".to_owned(),
        }),
        Some("ada@example.test".to_owned()),
        None,
        None,
    )
    .expect("a contact with a mobile and email should be created");
    contact.set_verify_state();

    assert_eq!(contact.phone_verified, None);
    assert_eq!(contact.mobile_verified, Some(VerifyState::None));
    assert_eq!(contact.email_verified, Some(VerifyState::None));
    assert_eq!(contact.address_verified, None);
}

#[test]
fn test_contact_model_metadata_preserves_source_indexes() {
    let metadata = metadata_of::<Contact>();
    assert!(metadata.primary_key().is_none());
    for field in [
        "phone",
        "phone_verified",
        "mobile",
        "mobile_verified",
        "email",
        "email_verified",
        "address",
        "address_verified",
    ] {
        assert!(
            metadata.indexes().any(|index| index.contains(field)),
            "missing index for {field}"
        );
    }
}

#[test]
fn test_contact_copy_verify_state_preserves_only_equal_values() {
    let mobile = Phone {
        country_area: None,
        city_area: None,
        number: "13800138000".to_owned(),
    };
    let source = Contact {
        phone: None,
        phone_verified: None,
        mobile: Some(mobile.clone()),
        mobile_verified: Some(VerifyState::Valid),
        email: Some("old@example.test".to_owned()),
        email_verified: Some(VerifyState::Valid),
        url: None,
        address: None,
        address_verified: None,
    };
    let mut target = Contact {
        mobile: Some(mobile),
        email: Some("new@example.test".to_owned()),
        ..Contact::default()
    };

    target.copy_verify_state(&source);

    assert_eq!(target.mobile_verified, Some(VerifyState::Valid));
    assert_eq!(target.email_verified, Some(VerifyState::None));
}

#[test]
fn test_contact_copy_verify_state_uses_source_address_identity() {
    let street = Info {
        id: Some(7),
        code: "street".into(),
        name: "Old display".into(),
        delete_time: None,
    };
    let old = Address {
        country: Info::default(),
        province: Info::default(),
        city: Info::default(),
        district: Info::default(),
        street: street.clone(),
        detail: "8 Main Road".into(),
        postalcode: Some("200000".into()),
        location: None,
    };
    let mut renamed_street = street;
    renamed_street.name = "New display".into();
    let current = Address {
        street: renamed_street,
        ..old.clone()
    };
    let source = Contact {
        address: Some(old),
        address_verified: Some(VerifyState::Valid),
        ..Contact::default()
    };
    let mut target = Contact {
        address: Some(current),
        ..Contact::default()
    };

    target.copy_verify_state(&source);

    assert_eq!(target.address_verified, Some(VerifyState::Valid));
}

#[test]
fn test_contact_normalize_recurses_and_clears_stale_verify_states() {
    let mut contact = Contact {
        phone: Some(Phone {
            country_area: Some(" 86 ".into()),
            city_area: None,
            number: "   ".into(),
        }),
        phone_verified: Some(VerifyState::Valid),
        mobile: None,
        mobile_verified: None,
        email: Some("   ".into()),
        email_verified: Some(VerifyState::Valid),
        url: Some(" https://example.test/path ".into()),
        address: None,
        address_verified: Some(VerifyState::Valid),
    };

    contact.normalize();

    assert_eq!(contact.phone, None);
    assert_eq!(contact.phone_verified, None);
    assert_eq!(contact.email, None);
    assert_eq!(contact.email_verified, None);
    assert_eq!(contact.url.as_deref(), Some("https://example.test/path"));
    assert_eq!(contact.address_verified, None);
}

#[test]
fn test_contact_redacts_email_and_nested_phone_number() {
    let contact = Contact {
        phone: None,
        phone_verified: None,
        mobile: Some(Phone {
            country_area: None,
            city_area: None,
            number: "13800138000".to_owned(),
        }),
        mobile_verified: None,
        email: Some("ada@example.test".to_owned()),
        email_verified: None,
        url: None,
        address: None,
        address_verified: None,
    };

    let redacted = format!("{:?}", contact.redacted());
    assert!(!redacted.contains("13800138000"));
    assert!(!redacted.contains("ada@example.test"));
}

#[test]
fn test_activity_coupon_redacts_nested_person_contact() {
    let person = Person {
        credential: Some(CredentialInfo {
            id: Some(1),
            r#type: CredentialType::IdentityCard,
            number: "320101199001010011".into(),
            verified: Some(VerifyState::Valid),
        }),
        contact: Contact::create(
            None,
            Some(Phone {
                country_area: None,
                city_area: None,
                number: "13800138000".into(),
            }),
            Some("private@example.test".into()),
            None,
            None,
        ),
        ..Person::default()
    };
    let now = Utc::now();
    let coupon = ActivityCoupon {
        id: None,
        activity: Info::default(),
        coupon_code: "CODE".into(),
        person,
        order: OrderInfo {
            id: None,
            user_id: None,
            app: StatefulInfo::default(),
            buyer: Buyer {
                id: None,
                user_id: None,
                name: String::new(),
                credential: None,
                gender: None,
                birthday: None,
                mobile: None,
                email: None,
            },
            seller: Seller {
                id: None,
                code: String::new(),
                name: String::new(),
                phone: None,
                mobile: None,
                email: None,
                url: None,
                credential: None,
                address: None,
            },
            source: None,
            category: None,
            pay_type: PayType::Medicare,
            currency: Currency::Cny,
            total_price: BigDecimal::default(),
            total_shipping_cost: BigDecimal::default(),
            total_discount: BigDecimal::default(),
            discount: BigDecimal::default(),
            shipping_cost: BigDecimal::default(),
            payable: BigDecimal::default(),
            status: OrderStatus::Submitted,
            create_time: now,
            modify_time: None,
            delete_time: None,
        },
        create_time: now,
        receive_time: now,
    };

    let redacted = format!("{:?}", coupon.redacted());
    assert!(!redacted.contains("13800138000"));
    assert!(!redacted.contains("private@example.test"));
    assert!(!redacted.contains("320101199001010011"));
    assert!(!redacted.contains("CODE"));
}
