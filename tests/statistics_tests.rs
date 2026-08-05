// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
};
use qubit_model::{
    china::{
        ChinaCities,
        ChinaDistricts,
        ChinaProvinces,
        IdentityCardUtils,
    },
    person::Gender,
    statistics::{
        CategoryValue,
        StatsDataset,
        StatsItem,
        TimeDimension,
    },
};
use qubit_model_metadata::{
    TemporalNormalization,
    TemporalPrecision,
    metadata_of,
};

/// Asserts that a type exposes model metadata.
fn assert_model<T: qubit_model_metadata::HasTypeMetadata>() {
    let _ = metadata_of::<T>();
}

#[test]
fn test_time_dimension_is_available() {
    assert_eq!(TimeDimension::Day, TimeDimension::Day);
}

/// Verifies every statistics type retains its Java source fields and derives
/// model metadata.
#[test]
fn test_statistics_models_preserve_source_fields_and_metadata() {
    let start_time = DateTime::from_timestamp(1_700_000_000, 0)
        .expect("the test timestamp should be representable");
    let item = StatsItem {
        name: "Orders".to_owned(),
        values: vec![
            BigDecimal::from_str("12.34").expect("the decimal should parse"),
            BigDecimal::from_str("56.78").expect("the decimal should parse"),
        ],
    };
    let category = CategoryValue {
        category: "Retail".to_owned(),
        value: BigDecimal::from(69),
        start_time: Some(start_time),
        end_time: None,
    };
    let dataset = StatsDataset {
        name: Some("Revenue".to_owned()),
        description: Some("Quarterly revenue".to_owned()),
        series: vec!["Q1".to_owned(), "Q2".to_owned()],
        items: vec![item.clone()],
    };

    assert_eq!(category.category, "Retail");
    assert_eq!(category.start_time, Some(start_time));
    assert_eq!(dataset.series.len(), 2);
    assert_eq!(dataset.items, vec![item]);
    assert_eq!(metadata_of::<CategoryValue>().struct_fields().len(), 4);
    assert_eq!(metadata_of::<StatsItem>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<StatsDataset>().struct_fields().len(), 4);
    assert_model::<TimeDimension>();
}

/// Verifies the Java precision and scale annotations survive as model
/// metadata.
#[test]
fn test_statistics_models_preserve_numeric_and_time_constraints() {
    for field_name in ["start_time", "end_time"] {
        let constraint = metadata_of::<CategoryValue>()
            .field(field_name)
            .expect("the time field should exist")
            .temporal_constraint()
            .expect("the time field should retain its precision");
        assert_eq!(constraint.precision(), TemporalPrecision::Second);
        assert_eq!(constraint.normalization(), TemporalNormalization::Utc);
    }

    let element = metadata_of::<StatsItem>()
        .field("values")
        .expect("the values field should exist")
        .element_metadata()
        .expect("the values field should constrain each decimal element");
    assert!(matches!(
        element.attributes(),
        [qubit_model_metadata::AttributeMetadata::Decimal(decimal)]
            if decimal.scale() == 2
    ));
}

/// Verifies the complete public identity-card layout constants.
#[test]
fn test_identity_card_layout_constants_match_the_source() {
    assert_eq!(IdentityCardUtils::NUMBER_LENGTH, 18);
    assert_eq!(IdentityCardUtils::AREA_INDEX, 0);
    assert_eq!(IdentityCardUtils::AREA_LENGTH, 6);
    assert_eq!(IdentityCardUtils::YEAR_INDEX, 6);
    assert_eq!(IdentityCardUtils::YEAR_LENGTH, 4);
    assert_eq!(IdentityCardUtils::MONTH_INDEX, 10);
    assert_eq!(IdentityCardUtils::MONTH_LENGTH, 2);
    assert_eq!(IdentityCardUtils::DAY_INDEX, 12);
    assert_eq!(IdentityCardUtils::DAY_LENGTH, 2);
    assert_eq!(IdentityCardUtils::SEQUENCE_INDEX, 14);
    assert_eq!(IdentityCardUtils::SEQUENCE_LENGTH, 3);
    assert_eq!(IdentityCardUtils::VERIFY_INDEX, 17);
}

/// Verifies checksum and calendar validation against the Java regression
/// corpus.
#[test]
fn test_identity_card_validate_matches_source_behavior() {
    for number in [
        "320114197001160058",
        "32128319931103141X",
        "32128319931103141x",
        "320121194905121510",
        "320121196612114711",
    ] {
        assert!(
            IdentityCardUtils::validate(number),
            "{number} should be valid"
        );
    }
    for number in [
        "",
        "320114197001160059",
        "32128319931103141y",
        "320114197013160058",
        "320114197002290058",
        "320114200002290058",
    ] {
        assert!(
            !IdentityCardUtils::validate(number),
            "{number} should be invalid"
        );
    }
}

/// Verifies extraction and birthday replacement behavior.
#[test]
fn test_identity_card_extracts_and_changes_encoded_values() {
    let male_number = "32128319931103141X";
    let female_number = "320114197001160066";

    assert_eq!(IdentityCardUtils::get_last_char(male_number), Some('X'));
    assert_eq!(
        IdentityCardUtils::get_birthday(male_number),
        NaiveDate::from_ymd_opt(1993, 11, 3)
    );
    assert_eq!(
        IdentityCardUtils::get_gender(male_number),
        Some(Gender::Male)
    );
    assert_eq!(
        IdentityCardUtils::get_gender(female_number),
        Some(Gender::Female)
    );
    assert_eq!(
        IdentityCardUtils::change_birthday(
            male_number,
            NaiveDate::from_ymd_opt(2000, 2, 29)
                .expect("the leap-day birthday should exist")
        ),
        Some("321283200002291416".to_owned())
    );
    assert_eq!(
        IdentityCardUtils::change_birthday("short", NaiveDate::MIN),
        None
    );
}

/// Verifies the bundled GB/T 2260 area data is complete and immutable to
/// callers.
#[test]
fn test_identity_card_area_lookup_uses_the_source_dataset() {
    let areas = IdentityCardUtils::get_area_map();

    assert_eq!(areas.len(), 3_219);
    assert_eq!(areas.get("110101"), Some(&"东城区"));
    assert_eq!(areas.get("659009"), Some(&"昆玉市"));
    assert!(IdentityCardUtils::is_area_valid("320114197001160058"));
    assert!(!IdentityCardUtils::is_area_valid("000000197001160058"));

    let district = IdentityCardUtils::get_district("320114197001160058")
        .expect("a known area code should produce a district");
    assert_eq!(district.code, "320114");
    assert_eq!(district.name, "雨花台区");
    assert!(IdentityCardUtils::get_district("000000197001160058").is_none());
}

/// Verifies the source marker classes also participate in model metadata.
#[test]
fn test_china_marker_models_derive_model_metadata() {
    assert_model::<ChinaCities>();
    assert_model::<ChinaDistricts>();
    assert_model::<ChinaProvinces>();
}
