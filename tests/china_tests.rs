// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

//! Behavioral coverage for Chinese resident identity-card utilities.

use chrono::NaiveDate;
use qubit_model::{
    china::IdentityCardUtils,
    person::Gender,
};

/// Exercises valid, malformed, and transformed resident identity numbers.
#[test]
fn test_identity_card_utils_validate_extract_and_change_known_numbers() {
    let number = "11010519491231002X";
    assert!(IdentityCardUtils::validate(number));
    assert_eq!(IdentityCardUtils::get_last_char(number), Some('X'));
    assert_eq!(
        IdentityCardUtils::get_birthday(number),
        Some(NaiveDate::from_ymd_opt(1949, 12, 31).expect("valid date"))
    );
    assert_eq!(IdentityCardUtils::get_gender(number), Some(Gender::Female));
    assert!(IdentityCardUtils::is_area_valid(number));
    assert!(IdentityCardUtils::get_district(number).is_some());
    let changed = IdentityCardUtils::change_birthday(
        number,
        NaiveDate::from_ymd_opt(2000, 2, 29).expect("leap date"),
    )
    .expect("valid identity number can change birthday");
    assert!(IdentityCardUtils::validate(&changed));

    for malformed in ["", "11010519491231002"] {
        assert!(!IdentityCardUtils::validate(malformed));
        assert_eq!(IdentityCardUtils::get_birthday(malformed), None);
        assert_eq!(IdentityCardUtils::get_district(malformed), None);
        assert!(!IdentityCardUtils::is_area_valid(malformed));
        assert_eq!(
            IdentityCardUtils::change_birthday(malformed, NaiveDate::MIN),
            None
        );
    }
    assert!(!IdentityCardUtils::validate("11010519490230002X"));
    assert_eq!(IdentityCardUtils::get_birthday("11010519490230002X"), None);
    assert_eq!(IdentityCardUtils::get_gender("1101051949123100AX"), None);
}
