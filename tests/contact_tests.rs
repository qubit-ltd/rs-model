// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

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

#[test]
fn test_phone_display_covers_optional_area_combinations() {
    assert_eq!(Phone::from("13800138000").to_string(), "13800138000");
    assert_eq!(
        Phone {
            country_area: Some("86".into()),
            city_area: None,
            number: "1".into()
        }
        .to_string(),
        "+86-1"
    );
    assert_eq!(
        Phone {
            country_area: None,
            city_area: Some("25".into()),
            number: "1".into()
        }
        .to_string(),
        "25-1"
    );
}
