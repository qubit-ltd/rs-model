// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for session-expiration reasons.

use qubit_model::system::ExpiredReason;

/// Maps every reason to its stable source identifier.
#[test]
fn test_expired_reason_ids_cover_all_variants() {
    assert_eq!(ExpiredReason::Logout.id(), "logout");
    assert_eq!(ExpiredReason::Timeout.id(), "timeout");
    assert_eq!(ExpiredReason::SingleSession.id(), "single_session");
    assert_eq!(ExpiredReason::Maintenance.id(), "maintenance");
    assert_eq!(ExpiredReason::None.id(), "none");
    assert_eq!(ExpiredReason::default(), ExpiredReason::None);
}
