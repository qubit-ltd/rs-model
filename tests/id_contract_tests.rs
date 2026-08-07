use qubit_id::Id;
use qubit_model::commons::AppResource;
use qubit_model::person::Person;

#[test]
fn test_domain_identifiers_are_non_optional_ids() {
    let person = Person {
        id: Id::from(7),
        ..Person::default()
    };
    let resource = AppResource {
        app_id: Id::from(11),
        resource_id: Id::from(12),
        ..AppResource::default()
    };

    assert_eq!(person.id, Id::from(7));
    assert_eq!(resource.app_id, Id::from(11));
    assert_eq!(resource.resource_id, Id::from(12));
}
