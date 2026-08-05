use qubit_model::ai::{AiResult, AiResultType};
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
fn test_ai_result_type_preserves_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&AiResultType::Description).expect("AI result type should serialize"),
        "\"DESCRIPTION\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Transcription)
            .expect("AI result type should serialize"),
        "\"TRANSCRIPTION\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Summary).expect("AI result type should serialize"),
        "\"SUMMARY\""
    );
    assert_eq!(
        serde_json::to_string(&AiResultType::Analysis).expect("AI result type should serialize"),
        "\"ANALYSIS\""
    );
}
