// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for logical relation symbols.

use qubit_model::system::LogicRelation;

/// Maps every relation to its expression syntax token.
#[test]
fn test_logic_relation_symbols_cover_all_variants() {
    assert_eq!(LogicRelation::And.symbol(), "AND");
    assert_eq!(LogicRelation::Or.symbol(), "OR");
    assert_eq!(LogicRelation::Not.symbol(), "NOT");
    assert_eq!(LogicRelation::default(), LogicRelation::And);
}
