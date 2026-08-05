// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use qubit_model::{
    ai::{
        AiResult,
        AiResultType,
    },
    upload::Attachment,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_ai_public_types_preserve_fields_and_traits() {
    assert_eq!(metadata_of::<AiResult>().struct_fields().len(), 13);
    assert_redact::<AiResult>();
    assert_redact::<AiResultType>();
}

#[test]
fn test_ai_result_preserves_indexes_reference_and_single_primary_key() {
    let metadata = metadata_of::<AiResult>();
    assert_eq!(
        metadata
            .primary_key()
            .expect("AI result primary key")
            .fields()
            .iter()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["id"]
    );
    let attachment = metadata
        .field("attachment_id")
        .unwrap()
        .reference()
        .expect("attachment reference");
    assert_eq!(
        attachment.target().identity().type_name(),
        core::any::type_name::<Attachment>()
    );
    for field in [
        "type",
        "language",
        "original_language",
        "engine_name",
        "engine_version",
        "process_start_time",
        "process_end_time",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(
            metadata.indexes().any(|index| index.contains(field)),
            "missing index for {field}"
        );
    }
}

#[test]
fn test_ai_result_type_preserves_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&AiResultType::Description)
            .expect("AI result type should serialize"),
        "\"DESCRIPTION\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Transcription)
            .expect("AI result type should serialize"),
        "\"TRANSCRIPTION\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Summary)
            .expect("AI result type should serialize"),
        "\"SUMMARY\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Analysis)
            .expect("AI result type should serialize"),
        "\"ANALYSIS\""
    );
}
