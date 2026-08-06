// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for seedable setting fixtures.

use qubit_model::setting::SettingRandomizer;

/// Generates deterministic valid settings within configured bounds.
#[test]
fn test_randomizer_is_seeded_valid_and_honors_ranges() {
    let mut first = SettingRandomizer::with_seed(42);
    let mut second = SettingRandomizer::with_seed(42);
    first.set_collection_size_range(2, 2);
    second.set_collection_size_range(2, 2);
    first.set_string_length_range(3, 3);
    second.set_string_length_range(3, 3);

    let generated = first.get();
    assert_eq!(generated, second.get());
    assert!(generated.is_valid());
    assert_eq!(generated.values.len(), 2);
    assert!(SettingRandomizer::SUPPORTED_TYPES.contains(&generated.data_type));
}

/// Rejects invalid fixture-generator ranges before storing them.
#[test]
fn test_randomizer_rejects_invalid_ranges() {
    let mut randomizer = SettingRandomizer::default();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            randomizer.set_collection_size_range(2, 1);
        }))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            randomizer.set_string_length_range(0, 1);
        }))
        .is_err()
    );
}
