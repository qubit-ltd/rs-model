//! Integration tests for the crate root exports.

use std::{path::PathBuf, process::Command};

#[test]
fn test_root_model_module_is_available() {
    let _: Option<qubit_model::ModelError> = None;
}

#[test]
fn test_model_error_preserves_validation_violation_context() {
    let error = qubit_model::ModelError::ValidationFailed {
        field: "name".to_owned(),
        reason: "must not be empty".to_owned(),
    };

    assert!(matches!(
        &error,
        qubit_model::ModelError::ValidationFailed { field, reason }
            if field == "name" && reason == "must not be empty"
    ));
    assert_eq!(
        error.to_string(),
        "model validation failed for field `name`: must not be empty"
    );
}

/// Verifies the inventory dependency scanner accepts the checked-in inventory.
#[test]
fn test_java_migration_dependency_scanner_validates_inventory() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let java_common_dir = manifest_dir.join("../../java-common");
    let output = Command::new("python3")
        .current_dir(&manifest_dir)
        .arg("scripts/synchronize_java_migration_dependencies.py")
        .arg("--check")
        .arg("--common-mixin-source")
        .arg(java_common_dir.join("common-mixin/src/main/java"))
        .arg("--common-model-source")
        .arg(java_common_dir.join("common-model/src/main/java"))
        .arg("--inventory")
        .arg("doc/java-migration-inventory.md")
        .output()
        .expect("the Python inventory dependency scanner should start");

    assert!(
        output.status.success(),
        "scanner validation failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
