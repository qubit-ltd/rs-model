//! Integration tests for the crate root exports.

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

#[test]
fn test_inventory_records_direct_name_builder_dependencies_and_nested_static_import() {
    let inventory = include_str!("../doc/java-migration-inventory.md");

    for expected_row in [
        "| `ltd.qubit.commons.mixin.WithEntity` | `qubit_mixin::WithEntity` | trait (Java interface) | ltd.qubit.commons.util.NameBuilder | available in qubit-mixin |",
        "| `ltd.qubit.model.commons.Owner` | `qubit_model::commons::Owner` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.util.NameBuilder | planned |",
        "| `ltd.qubit.model.task.TaskInfo` | `qubit_model::task::TaskInfo` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.commons.mixin.WithStatus, ltd.qubit.commons.util.NameBuilder, ltd.qubit.model.commons.Category, ltd.qubit.model.mixin.WithCreator, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |",
        "| `ltd.qubit.model.system.SettingXmlAdapter` | `qubit_model::system::SettingXmlAdapter` | struct (Java class) | static ltd.qubit.model.system.SettingXmlAdapter.Adapted | planned |",
    ] {
        assert!(
            inventory.contains(expected_row),
            "missing inventory row: {expected_row}"
        );
    }
}
