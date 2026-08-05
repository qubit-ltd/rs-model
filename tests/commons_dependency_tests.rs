// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::commons::DictEntryInfo;

#[test]
fn test_dict_entry_info_serializes_source_field_names() {
    let entry = DictEntryInfo {
        id: Some(3),
        code: "frequency".to_owned(),
        name: "Frequency".to_owned(),
        dict_id: Some(2),
        params: vec!["daily".to_owned()],
        delete_time: None,
    };
    let value = serde_json::to_value(entry).expect("dictionary entry should serialize");
    assert_eq!(value["dict_id"], 2);
    assert_eq!(value["params"][0], "daily");
}
