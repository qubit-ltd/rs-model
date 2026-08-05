//! Integration tests for migrated contact domain models.

use qubit_model::contact::Phone;

#[test]
fn test_phone_preserves_all_source_number_components() {
    let phone = Phone {
        country_area: Some("86".to_owned()),
        city_area: Some("025".to_owned()),
        number: "88273847".to_owned(),
    };

    assert_eq!(phone.country_area.as_deref(), Some("86"));
    assert_eq!(phone.city_area.as_deref(), Some("025"));
    assert_eq!(phone.number, "88273847");
}
