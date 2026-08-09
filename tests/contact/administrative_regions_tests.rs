// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration-test mirror for the corresponding public model module.

use qubit_model::contact::City;
use qubit_model::contact::Country;
use qubit_model::contact::District;
use qubit_model::contact::Province;
use qubit_model::contact::Street;
use qubit_model_metadata::metadata_of;

/// Keeps the source-to-test mapping explicit while shared model contract tests
/// exercise serialization, metadata, and redaction behavior.
#[test]
fn test_administrative_regions_tests_mirror() {
    assert!(
        !module_path!().is_empty(),
        "the test module path must be available"
    );
}

#[test]
fn test_administrative_regions_preserve_java_keys_indexes_and_references() {
    for metadata in [
        metadata_of::<Country>(),
        metadata_of::<Province>(),
        metadata_of::<City>(),
        metadata_of::<District>(),
        metadata_of::<Street>(),
    ] {
        assert_eq!(metadata.primary_key().unwrap().fields()[0].name(), "id");
        assert!(
            metadata
                .unique_constraints()
                .any(|unique| unique.contains("code"))
        );
        assert!(metadata.indexes().any(|index| index.contains("name")));
        for field in ["predefined", "create_time", "modify_time", "delete_time"] {
            assert!(metadata.indexes().any(|index| index.contains(field)));
        }
    }

    let province = metadata_of::<Province>();
    assert!(province.field("country").unwrap().reference().is_some());
    for field in ["country", "postalcode", "level"] {
        assert!(province.indexes().any(|index| index.contains(field)));
    }

    let city = metadata_of::<City>();
    assert!(city.field("province").unwrap().reference().is_some());
    for field in ["province", "phone_area", "postalcode", "level"] {
        assert!(city.indexes().any(|index| index.contains(field)));
    }

    let district = metadata_of::<District>();
    assert!(district.field("city").unwrap().reference().is_some());
    for field in ["city", "postalcode", "level"] {
        assert!(district.indexes().any(|index| index.contains(field)));
    }

    let street = metadata_of::<Street>();
    assert!(street.field("district").unwrap().reference().is_some());
    for field in ["district", "postalcode", "level"] {
        assert!(street.indexes().any(|index| index.contains(field)));
    }
}
