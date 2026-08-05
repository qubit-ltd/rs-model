use qubit_model::appointment::Appointment;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_appointment_preserves_all_java_fields_and_traits() {
    assert_eq!(metadata_of::<Appointment>().struct_fields().len(), 11);
    assert_redact::<Appointment>();
}
