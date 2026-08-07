// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::NaiveTime;

use qubit_model::thirdpart::WechatJsConfig;
use qubit_model::work::LocalTimeRange;
use qubit_model::work::WorkSchedule;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn work_and_thirdpart_models_preserve_source_shapes() {
    assert_redact::<LocalTimeRange>();
    assert_redact::<WorkSchedule>();
    assert_redact::<WechatJsConfig>();
    assert_eq!(metadata_of::<LocalTimeRange>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<WorkSchedule>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<WechatJsConfig>().struct_fields().len(), 4);
    assert_eq!(
        metadata_of::<WorkSchedule>()
            .primary_key()
            .unwrap()
            .fields()[0]
            .name(),
        "id"
    );
}

#[test]
fn local_time_range_preserves_half_open_membership() {
    let range = LocalTimeRange {
        start: Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
        end: Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap()),
    };
    assert!(range.contains(NaiveTime::from_hms_opt(9, 0, 0).unwrap()));
    assert!(!range.contains(NaiveTime::from_hms_opt(17, 0, 0).unwrap()));
    assert!(LocalTimeRange::default().is_empty());
}

#[test]
fn thirdpart_redaction_hides_nonce_and_signature() {
    let config = WechatJsConfig {
        app_id: "wx-app".into(),
        timestamp: "123".into(),
        nonce_str: "private-nonce".into(),
        signature: "private-signature".into(),
    };
    let rendered = format!("{:?}", config.redacted());
    assert!(!rendered.contains("private-nonce"));
    assert!(!rendered.contains("private-signature"));
    let wire = serde_json::to_value(config).unwrap();
    assert_eq!(wire["app_id"], "wx-app");
    assert_eq!(wire["nonce_str"], "private-nonce");
}
