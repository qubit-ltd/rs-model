//! Integration tests for the crate root exports.

use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn test_root_model_module_is_available() {
    let _: Option<qubit_model::ModelError> = None;
}

#[test]
fn test_model_error_preserves_validation_violation_context() {
    let error = qubit_model::ModelError::ValidationFailed {
        message: Some("name is required".to_owned()),
        violations: vec![qubit_model::ValidationViolation {
            field: "name".to_owned(),
            reason: "must not be empty".to_owned(),
        }],
    };

    assert!(matches!(
        &error,
        qubit_model::ModelError::ValidationFailed { message: Some(message), violations }
            if message == "name is required"
                && violations == &[qubit_model::ValidationViolation {
                    field: "name".to_owned(),
                    reason: "must not be empty".to_owned(),
                }]
    ));
    assert_eq!(error.to_string(), "name is required");
}

/// Verifies a validation failure can represent an empty violation set.
#[test]
fn test_model_error_represents_zero_validation_violations() {
    let error = qubit_model::ModelError::ValidationFailed {
        message: None,
        violations: Vec::new(),
    };

    assert!(matches!(
        error,
        qubit_model::ModelError::ValidationFailed { message: None, violations }
            if violations.is_empty()
    ));
}

/// Verifies a validation failure retains multiple structured violations.
#[test]
fn test_model_error_represents_multiple_validation_violations() {
    let error = qubit_model::ModelError::ValidationFailed {
        message: None,
        violations: vec![
            qubit_model::ValidationViolation {
                field: "name".to_owned(),
                reason: "must not be empty".to_owned(),
            },
            qubit_model::ValidationViolation {
                field: "email".to_owned(),
                reason: "must be a valid email address".to_owned(),
            },
        ],
    };

    assert!(matches!(
        error,
        qubit_model::ModelError::ValidationFailed { message: None, violations }
            if violations.len() == 2
    ));
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

/// Verifies scanner validation rejects both missing and stale public declarations.
#[test]
fn test_java_migration_dependency_scanner_rejects_inventory_declaration_mismatches() {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the system clock should be after the Unix epoch")
        .as_nanos();
    let test_dir = std::env::temp_dir().join(format!(
        "qubit-model-java-inventory-{unique_suffix}-{}",
        std::process::id()
    ));
    let common_mixin_source = test_dir.join("common-mixin");
    let common_model_source = test_dir.join("common-model");
    fs::create_dir_all(&common_mixin_source)
        .expect("the mixin fixture directory should be created");
    fs::create_dir_all(&common_model_source)
        .expect("the model fixture directory should be created");
    fs::write(
        common_mixin_source.join("VisibleMixin.java"),
        "package test.mixin; public interface VisibleMixin {}",
    )
    .expect("the mixin fixture should be written");
    fs::write(
        common_model_source.join("VisibleModel.java"),
        "package test.model; public class VisibleModel {\n\
         String brace = \"}\";\n\
         char open = '{';\n\
         String block = \"\"\" { ignored } \"\"\";\n\
         public static class NestedVisible {}\n\
         }",
    )
    .expect("the model fixture should be written");
    let inventory = test_dir.join("inventory.md");
    fs::write(
        &inventory,
        "| `test.mixin.VisibleMixin` | `qubit_model::VisibleMixin` | trait | - | planned |\n\
         | `test.model.RemovedModel` | `qubit_model::RemovedModel` | struct | - | planned |\n",
    )
    .expect("the inventory fixture should be written");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new("python3")
        .current_dir(&manifest_dir)
        .arg("scripts/synchronize_java_migration_dependencies.py")
        .arg("--check")
        .arg("--common-mixin-source")
        .arg(&common_mixin_source)
        .arg("--common-model-source")
        .arg(&common_model_source)
        .arg("--inventory")
        .arg(&inventory)
        .output()
        .expect("the Python inventory dependency scanner should start");
    fs::remove_dir_all(&test_dir).expect("the fixture directory should be removed");

    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.status.success(),
        "scanner should reject declaration drift"
    );
    assert!(
        diagnostics.contains("missing from inventory"),
        "scanner should report added public declarations: {diagnostics}"
    );
    assert!(
        diagnostics.contains("test.model.VisibleModel"),
        "scanner should report the added public top-level declaration: {diagnostics}"
    );
    assert!(
        diagnostics.contains("test.model.VisibleModel.NestedVisible"),
        "scanner should report the added public nested declaration: {diagnostics}"
    );
    assert!(
        diagnostics.contains("missing from source"),
        "scanner should report deleted public declarations: {diagnostics}"
    );
    assert!(
        diagnostics.contains("test.model.RemovedModel"),
        "scanner should report the deleted public declaration: {diagnostics}"
    );
}
