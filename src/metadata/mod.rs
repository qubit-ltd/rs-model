// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Scoped metadata for categories, dictionaries, aggregate payloads, and acquisition sources.

mod aggregate_ref;
mod category;
mod dict;
mod dict_entry;
mod dict_entry_info;
mod full_dict;
mod payload;
mod scope;
mod scope_type;
mod source;

pub use aggregate_ref::AggregateRef;
pub use category::Category;
pub use dict::Dict;
pub use dict_entry::DictEntry;
pub use dict_entry_info::DictEntryInfo;
pub use full_dict::FullDict;
pub use payload::Payload;
pub use scope::Scope;
pub use scope_type::ScopeType;
pub use source::Source;
