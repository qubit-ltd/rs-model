// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_mixin::{Emptyful, Normalizable};
use qubit_model::{
    commons::State,
    metadata::{
        AggregateRef, Category, Dict, DictEntry, DictEntryInfo, FullDict, Payload, Scope,
        ScopeType, Source,
    },
};
use qubit_model_metadata::{metadata_of, UniqueComparison};
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

/// Exercises the public empty and normalization trait dispatch for one
/// metadata value.
fn exercise_metadata_traits<T: Emptyful + Normalizable>(value: &mut T) {
    let _ = Emptyful::is_empty(&*value);
    let _ = Normalizable::is_normalized_empty(&*value);
    value.normalize();
}

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
    assert!(category
        .unique_constraints()
        .any(|unique| unique.contains("code")));
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
    assert!(payload
        .unique_constraints()
        .any(|unique| unique.contains("key") && unique.contains("aggregate_ref")));
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

/// Verifies every metadata aggregate exposes its source-compatible default,
/// normalization, emptiness, and JSON contracts.
#[test]
fn metadata_aggregates_preserve_default_and_normalization_contracts() {
    let mut category = Category::default();
    assert!(category.is_empty());
    exercise_metadata_traits(&mut category);
    category.code = "  MEDICAL  ".into();
    category.name = "  Medical  ".into();
    category.normalize();
    assert_eq!(category.code, "MEDICAL");
    assert_eq!(category.name, "Medical");
    assert!(!category.is_empty());
    let category_json = serde_json::to_value(&category)
        .expect("a category should serialize to its public JSON contract");
    let restored_category: Category =
        serde_json::from_value(category_json).expect("a category JSON value should deserialize");
    assert_eq!(restored_category, category);

    let mut source = Source::default();
    assert!(source.is_empty());
    exercise_metadata_traits(&mut source);
    source.code = "  API  ".into();
    source.name = "  Partner API  ".into();
    source.entity = "  CLAIM  ".into();
    source.normalize();
    assert_eq!(source.code, "API");
    assert_eq!(source.name, "Partner API");
    assert_eq!(source.entity, "CLAIM");
    assert!(!source.is_empty());
    let source_json = serde_json::to_value(&source)
        .expect("a source should serialize to its public JSON contract");
    let restored_source: Source =
        serde_json::from_value(source_json).expect("a source JSON value should deserialize");
    assert_eq!(restored_source, source);

    let mut dict = Dict::default();
    exercise_metadata_traits(&mut dict);
    dict.code = "  BENEFIT  ".into();
    dict.name = "  Benefit  ".into();
    dict.description = Some("  Catalog  ".into());
    dict.normalize();
    assert_eq!(dict.code, "BENEFIT");
    assert_eq!(dict.name, "Benefit");
    assert_eq!(dict.description.as_deref(), Some("Catalog"));
    let full_dict = FullDict::from(dict.clone());
    assert_eq!(full_dict.code, dict.code);
    assert_eq!(full_dict.entries, None);
    let dict_json = serde_json::to_value(&dict)
        .expect("a dictionary should serialize to its public JSON contract");
    let restored_dict: Dict =
        serde_json::from_value(dict_json).expect("a dictionary JSON value should deserialize");
    assert_eq!(restored_dict, dict);

    let mut full_dict = FullDict::default();
    exercise_metadata_traits(&mut full_dict);
    assert!(full_dict.is_empty());
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

    let mut assigned = DictEntry::default();
    assigned.assign_info(&DictEntryInfo {
        id: Some(7),
        code: "  {0}D  ".into(),
        name: "  Every {0} days  ".into(),
        dict_id: Some(3),
        params: Some(vec!["2".into()]),
        delete_time: None,
    });
    assert_eq!(assigned.id, Some(7));
    assert_eq!(assigned.display_code(&["5"]), "  5D  ");
    assert_eq!(assigned.display_name(&["5"]), "  Every 5 days  ");
    assigned.normalize();
    assert_eq!(assigned.code, "{0}D");
    assert!(!assigned.is_empty());
    assert_eq!(assigned.info().dict_id, Some(3));
    assert_eq!(assigned.info().params, None);
    exercise_metadata_traits(&mut assigned);
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
    assert!(serde_json::to_value(&no_params)
        .unwrap()
        .get("params")
        .is_none());

    let payload = Payload::default();
    assert!(payload.is_empty());
    let sensitive = Payload::new("token", Some("raw-secret".into()));
    assert!(!format!("{:?}", sensitive.redacted()).contains("raw-secret"));

    let empty_info = DictEntryInfo::default();
    assert!(empty_info.is_empty());
    let created = DictEntryInfo::create(Some(5), None, None)
        .expect("an identifier should create dictionary entry info");
    assert_eq!(created.id, Some(5));
    assert_eq!(created.code, "");
    assert_eq!(created.name, "");

    let aggregate = AggregateRef::default();
    assert!(aggregate.is_empty());
    let aggregate_json =
        serde_json::to_value(&aggregate).expect("an aggregate reference should serialize");
    let restored_aggregate: AggregateRef = serde_json::from_value(aggregate_json)
        .expect("an aggregate reference JSON value should deserialize");
    assert_eq!(restored_aggregate, aggregate);

    let scope = Scope {
        r#type: ScopeType::Tenant,
        id: Some(42),
    };
    let scope_json = serde_json::to_value(&scope).expect("a scope should serialize");
    let restored_scope: Scope =
        serde_json::from_value(scope_json).expect("a scope JSON value should deserialize");
    assert_eq!(restored_scope, scope);

    let mut aggregate_for_traits = AggregateRef::default();
    exercise_metadata_traits(&mut aggregate_for_traits);
    let mut payload_for_traits = Payload::default();
    exercise_metadata_traits(&mut payload_for_traits);
}
