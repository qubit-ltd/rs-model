// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for audited operation actions.

use qubit_model::system::Action;

/// Preserves every source action's wire name and display label.
#[test]
fn test_action_display_names_and_wire_lookup_cover_all_variants() {
    for action in [
        Action::Login,
        Action::Logout,
        Action::List,
        Action::Get,
        Action::Add,
        Action::Update,
        Action::Delete,
        Action::Restore,
        Action::Purge,
        Action::PurgeAll,
        Action::Erase,
        Action::BatchAdd,
        Action::BatchUpdate,
        Action::BatchDelete,
        Action::BatchRestore,
        Action::BatchPurge,
        Action::BatchErase,
        Action::Clear,
        Action::Import,
        Action::Export,
        Action::TestExistence,
        Action::Bind,
        Action::Register,
        Action::Reset,
        Action::Check,
        Action::Unregister,
        Action::Unbound,
        Action::Send,
        Action::Authenticate,
        Action::Refresh,
        Action::Count,
        Action::ListAll,
        Action::ListFirst,
        Action::ForEach,
        Action::AddOrUpdate,
        Action::PerformAction,
    ] {
        assert!(!action.display_name().is_empty());
        let name = serde_json::to_value(action)
            .expect("actions are serializable")
            .as_str()
            .expect("action wire value is textual")
            .to_owned();
        assert_eq!(Action::from_name(&name), Some(action));
    }
    assert_eq!(Action::default(), Action::Get);
    assert_eq!(Action::from_name("unknown"), None);
}
