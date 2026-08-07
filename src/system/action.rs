// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Audited operation actions.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Common action recorded by an operation log.
#[derive(Model, Redact, Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
    /// Logs a user in.
    Login,
    /// Logs a user out.
    Logout,
    /// Lists filtered objects.
    List,
    /// Gets one object's details.
    #[default]
    Get,
    /// Adds an object.
    Add,
    /// Updates an object.
    Update,
    /// Marks an object as deleted.
    Delete,
    /// Restores a marked-deleted object.
    Restore,
    /// Purges a marked-deleted object.
    Purge,
    /// Purges all marked-deleted objects.
    PurgeAll,
    /// Erases an object regardless of deletion state.
    Erase,
    /// Adds multiple objects.
    BatchAdd,
    /// Updates multiple objects.
    BatchUpdate,
    /// Marks multiple objects as deleted.
    BatchDelete,
    /// Restores multiple marked-deleted objects.
    BatchRestore,
    /// Purges multiple marked-deleted objects.
    BatchPurge,
    /// Erases multiple objects regardless of deletion state.
    BatchErase,
    /// Clears all objects.
    Clear,
    /// Imports objects.
    Import,
    /// Exports objects.
    Export,
    /// Tests whether an object exists.
    TestExistence,
    /// Binds an object.
    Bind,
    /// Registers an object.
    Register,
    /// Resets an object.
    Reset,
    /// Checks an object.
    Check,
    /// Unregisters an object.
    Unregister,
    /// Unbinds an object.
    Unbound,
    /// Sends an object.
    Send,
    /// Authenticates an object.
    Authenticate,
    /// Refreshes an object.
    Refresh,
    /// Counts objects.
    Count,
    /// Lists all objects.
    ListAll,
    /// Lists the first object.
    ListFirst,
    /// Performs an operation for each object.
    ForEach,
    /// Adds a new object or updates an existing object.
    AddOrUpdate,
    /// Performs an action on an object.
    PerformAction,
}

impl Action {
    /// Returns the source display name used in log messages.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Login => "Login",
            Self::Logout => "Logout",
            Self::List => "List",
            Self::Get => "Get",
            Self::Add => "Add",
            Self::Update => "Update",
            Self::Delete => "Delete",
            Self::Restore => "Restore",
            Self::Purge => "Purge",
            Self::PurgeAll => "Purge all",
            Self::Erase => "Erase",
            Self::BatchAdd => "Batch add",
            Self::BatchUpdate => "Batch update",
            Self::BatchDelete => "Batch delete",
            Self::BatchRestore => "Batch restore",
            Self::BatchPurge => "Batch purge",
            Self::BatchErase => "Batch erase",
            Self::Clear => "Clear all",
            Self::Import => "Import",
            Self::Export => "Export",
            Self::TestExistence => "Test existence of",
            Self::Bind => "Bind",
            Self::Register => "Register",
            Self::Reset => "Reset",
            Self::Check => "Check",
            Self::Unregister => "Unregister",
            Self::Unbound => "Unbound",
            Self::Send => "Send",
            Self::Authenticate => "Authenticate",
            Self::Refresh => "Refresh",
            Self::Count => "Count",
            Self::ListAll => "List all",
            Self::ListFirst => "List first",
            Self::ForEach => "For each",
            Self::AddOrUpdate => "Add or update",
            Self::PerformAction => "Perform action on",
        }
    }

    /// Looks up an action by its source enum name.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        serde_json::from_value(serde_json::Value::String(name.to_owned())).ok()
    }
}
