use qubit_model::{
    activity::{Activity, ActivityCoupon, ActivityProductItem},
    commons::VerifyState,
    contact::{Contact, Phone},
    person::Person,
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
