// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_mixin::Normalizable;
use qubit_model::{
    commons::State,
    metadata::{
        AggregateRef,
        Category,
        Dict,
        DictEntry,
        DictEntryInfo,
        FullDict,
        Payload,
        Scope,
        ScopeType,
        Source,
    },
};
use qubit_model_metadata::{
    UniqueComparison,
    metadata_of,
};
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
    assert_eq!(metadata_of::<FullDict>().struct_fields().len(), 16);
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
    assert_eq!(
        category
            .unique_constraints()
            .find(|unique| unique.contains("code"))
            .unwrap()
            .comparison_of("code"),
        Some(UniqueComparison::IgnoreCase)
    );
    assert_eq!(
        category
            .unique_constraints()
            .find(|unique| unique.contains("name"))
            .unwrap()
            .comparison_of("name"),
        Some(UniqueComparison::IgnoreCase)
    );

    let dict_entry = metadata_of::<DictEntry>();
    assert!(dict_entry.field("dict").unwrap().reference().is_some());
    assert!(dict_entry.field("parent").unwrap().reference().is_some());
    assert_eq!(
        dict_entry
            .unique_constraints()
            .next()
            .unwrap()
            .comparison_of("code"),
        Some(UniqueComparison::IgnoreCase)
    );

    let payload = metadata_of::<Payload>();
    assert!(payload.unique_constraints().any(
        |unique| unique.contains("key") && unique.contains("aggregate_ref")
    ));
    assert_eq!(
        payload
            .unique_constraints()
            .next()
            .unwrap()
            .comparison_of("key"),
        Some(UniqueComparison::IgnoreCase)
    );

    let aggregate_ref = metadata_of::<AggregateRef>();
    assert_eq!(
        aggregate_ref.keys().next().unwrap().fields(),
        &["type", "id", "property"]
    );
    let scope = metadata_of::<Scope>();
    assert!(scope.indexes().any(|index| index.contains("type")));
    assert!(scope.indexes().any(|index| index.contains("id")));

    let full_dict = metadata_of::<FullDict>();
    assert_eq!(full_dict.primary_key().unwrap().fields()[0].name(), "id");
    assert!(full_dict.indexes().any(|index| index.contains("scope")));
}

#[test]
fn metadata_leaf_types_preserve_source_wire_contracts() {
    let aggregate = serde_json::to_value(AggregateRef {
        r#type: "PERSON".into(),
        id: Some(7),
        property: Some("ATTACHMENTS".into()),
    })
    .unwrap();
    assert_eq!(aggregate["type"], "PERSON");
    assert!(aggregate.get("entityType").is_none());

    let scope = serde_json::to_value(Scope {
        r#type: ScopeType::Organization,
        id: Some(9),
    })
    .unwrap();
    assert_eq!(scope["type"], "ORGANIZATION");
    assert!(scope.get("scopeType").is_none());

    let source = serde_json::to_value(Source::default()).unwrap();
    assert!(source.get("id").is_none());
    assert!(source.get("create_time").is_none());
    assert!(source.get("createTime").is_none());
}

#[test]
fn dict_defaults_to_the_source_normal_state() {
    let dict = Dict::default();

    assert_eq!(dict.state, State::Normal);
    assert!(dict.is_empty());
}

#[test]
fn metadata_models_normalize_strings_and_use_all_fields_for_emptiness() {
    let mut aggregate = AggregateRef {
        r#type: "  PERSON  ".into(),
        id: None,
        property: Some("   ".into()),
    };
    aggregate.normalize();
    assert_eq!(aggregate.r#type, "PERSON");
    assert_eq!(aggregate.property, None);

    let mut entry = DictEntryInfo {
        code: "  CODE  ".into(),
        name: "  Name  ".into(),
        params: Some(Vec::new()),
        ..DictEntryInfo::default()
    };
    entry.normalize();
    assert_eq!(entry.code, "CODE");
    assert_eq!(entry.name, "Name");
    assert_eq!(entry.params, None);

    let dict = Dict {
        description: Some("present".into()),
        ..Dict::default()
    };
    assert!(!dict.is_empty());
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

    let plain = DictEntry::new("READY", "Ready");
    assert_eq!(
        plain.match_code_and_format_name("ready").as_deref(),
        Some("Ready")
    );
}

#[test]
fn full_dict_translates_exact_dirty_and_parameterized_codes() {
    let full = FullDict {
        id: None,
        code: String::new(),
        name: String::new(),
        scope: None,
        standard_doc: None,
        standard_code: None,
        url: None,
        description: None,
        comment: None,
        category: None,
        state: State::Normal,
        predefined: false,
        create_time: None,
        modify_time: None,
        delete_time: None,
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
        params: Some(vec!["1".into(), "2".into()]),
        delete_time: None,
    };
    assert_eq!(info.display_code(), "1W2D");
    assert_eq!(info.display_name(), "每1星期使用2天");
    assert!(DictEntryInfo::create(None, None, None).is_none());
    let serialized = serde_json::to_value(&info).unwrap();
    assert_eq!(serialized["display_code"], "1W2D");
    assert_eq!(serialized["display_name"], "每1星期使用2天");

    let no_params = DictEntryInfo {
        params: None,
        ..info.clone()
    };
    assert_eq!(no_params.display_code(), "{0}W{1}D");
    assert!(
        serde_json::to_value(&no_params)
            .unwrap()
            .get("params")
            .is_none()
    );

    let payload = Payload::default();
    assert!(payload.is_empty());
    let sensitive = Payload::new("token", Some("raw-secret".into()));
    assert!(!format!("{:?}", sensitive.redacted()).contains("raw-secret"));
}
