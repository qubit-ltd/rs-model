// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use qubit_model::{
    commons::State,
    metadata::{
        AggregateRef, Category, Dict, DictEntry, DictEntryInfo, FullDict, Payload, Scope,
        ScopeType, Source,
    },
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn metadata_public_types_preserve_source_fields_and_traits() {
    assert_redact::<AggregateRef>();
    assert_redact::<Category>();
    assert_redact::<Dict>();
    assert_redact::<DictEntry>();
    assert_redact::<DictEntryInfo>();
    assert_redact::<FullDict>();
    assert_redact::<Payload>();
    assert_redact::<Scope>();
    assert_redact::<ScopeType>();
    assert_redact::<Source>();

    assert_eq!(metadata_of::<Category>().struct_fields().len(), 13);
    assert_eq!(metadata_of::<Dict>().struct_fields().len(), 15);
    assert_eq!(metadata_of::<DictEntry>().struct_fields().len(), 10);
    assert_eq!(metadata_of::<DictEntryInfo>().struct_fields().len(), 6);
    assert_eq!(metadata_of::<FullDict>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<Payload>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Source>().struct_fields().len(), 11);
}

#[test]
fn metadata_model_constraints_preserve_source_annotations() {
    let category = metadata_of::<Category>();
    assert_eq!(category.primary_key().unwrap().fields()[0].name(), "id");
    assert!(
        category
            .unique_constraints()
            .any(|unique| unique.contains("code"))
    );
    assert!(category.indexes().any(|index| index.contains("scope")));
    assert!(category.field("parent").unwrap().reference().is_some());

    let dict_entry = metadata_of::<DictEntry>();
    assert!(dict_entry.field("dict").unwrap().reference().is_some());
    assert!(dict_entry.field("parent").unwrap().reference().is_some());

    let payload = metadata_of::<Payload>();
    assert!(
        payload
            .unique_constraints()
            .any(|unique| unique.contains("key") && unique.contains("aggregate_ref"))
    );
}

#[test]
fn dict_defaults_to_the_source_normal_state() {
    let dict = Dict::default();

    assert_eq!(dict.state, State::Normal);
    assert!(dict.is_empty());
}

#[test]
fn dict_entry_formats_and_matches_parameterized_codes() {
    let entry = DictEntry::new("{0}W{1}D", "每{0}星期使用{1}天");

    assert!(entry.has_parameter());
    assert_eq!(entry.display_code(&["1", "2"]), "1W2D");
    assert_eq!(entry.display_name(&["1", "2"]), "每1星期使用2天");
    assert_eq!(
        entry.match_code_and_format_name("3w4d").as_deref(),
        Some("每3星期使用4天")
    );
    assert_eq!(entry.match_code_and_format_name("3-W4D"), None);
}

#[test]
fn full_dict_translates_exact_dirty_and_parameterized_codes() {
    let full = FullDict {
        dict: Dict::default(),
        entries: Some(vec![
            DictEntry::new("A", "Alpha"),
            DictEntry::new("1", "One"),
            DictEntry::new("AC{0}H", "饭前{0}小时"),
        ]),
    };

    assert_eq!(full.translate("a").as_deref(), Some("Alpha"));
    assert_eq!(full.translate(" A ").as_deref(), Some("Alpha"));
    assert_eq!(full.translate("0001").as_deref(), Some("One"));
    assert_eq!(full.translate("ac2h").as_deref(), Some("饭前2小时"));
    assert_eq!(full.translate("missing"), None);
}

#[test]
fn dict_entry_info_and_payload_preserve_computed_behaviors() {
    let info = DictEntryInfo {
        id: Some(9),
        code: "{0}W{1}D".into(),
        name: "每{0}星期使用{1}天".into(),
        dict_id: Some(3),
        params: vec!["1".into(), "2".into()],
        delete_time: None,
    };
    assert_eq!(info.display_code(), "1W2D");
    assert_eq!(info.display_name(), "每1星期使用2天");
    assert!(DictEntryInfo::create(None, None, None).is_none());

    let payload = Payload::default();
    assert!(payload.is_empty());
    let sensitive = Payload::new("token", Some("raw-secret".into()));
    assert!(!format!("{:?}", sensitive.redacted()).contains("raw-secret"));
}
