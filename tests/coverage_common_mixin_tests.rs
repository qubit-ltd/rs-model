// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioural coverage for common values and composed model mixins.

use chrono::Utc;
use qubit_id::Id;

use qubit_mixin::Emptyful;
use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model::commons::Code;
use qubit_model::commons::CodeMap;
use qubit_model::commons::CredentialInfo;
use qubit_model::commons::CredentialInfoCodec;
use qubit_model::commons::CredentialInfoCodecError;
use qubit_model::commons::CredentialType;
use qubit_model::commons::Faq;
use qubit_model::commons::Schedule;
use qubit_model::commons::State;
use qubit_model::commons::Token;
use qubit_model::mixin::HasStatefulInfo;
use qubit_model::mixin::InfoWithAppEntity;
use qubit_model::mixin::InfoWithToken;
use qubit_model::mixin::Stateful;
use qubit_model::mixin::StatefulInfo;
use qubit_model::mixin::WithApp;
use qubit_model::mixin::WithStatefulInfoWithToken;
use qubit_model::mixin::WithToken;

/// Exercises source empty and normalization traits for a model value.
fn exercise_traits<T: Emptyful + Normalizable>(value: &mut T) {
    let _ = Emptyful::is_empty(&*value);
    let _ = Normalizable::is_normalized_empty(&*value);
    value.normalize();
}

/// A minimal external consumer of the composed stateful-token projection.
#[derive(Default)]
struct Projection {
    /// The lifecycle snapshot exposed by the source interface.
    info: StatefulInfo,
    /// The optional current access token.
    token: Option<Token>,
}

impl Stateful for Projection {
    fn state(&self) -> Option<State> {
        self.info.state
    }

    fn set_state(&mut self, state: Option<State>) {
        self.info.state = state;
    }
}

impl HasStatefulInfo for Projection {
    fn stateful_info(&self) -> StatefulInfo {
        self.info.clone()
    }
}

impl WithToken for Projection {
    fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn set_token(&mut self, token: Option<Token>) {
        self.token = token;
    }
}

impl WithStatefulInfoWithToken for Projection {}

/// Verifies every Java credential name is accepted and emitted without aliases.
#[test]
fn test_credential_info_codec_round_trips_every_credential_type() {
    let variants = [
        CredentialType::IdentityCard,
        CredentialType::ResidenceBooklet,
        CredentialType::Passport,
        CredentialType::OfficerCard,
        CredentialType::DrivingCard,
        CredentialType::HongkongMacaoReturnPermit,
        CredentialType::TaiwanReturnPermit,
        CredentialType::PoliceCard,
        CredentialType::HongkongPassport,
        CredentialType::MacaoPassport,
        CredentialType::TaiwanPassport,
        CredentialType::ForeignerPermanentResidencePermit,
        CredentialType::HongkongMacaoTaiwanResidencePermit,
        CredentialType::BirthCertificate,
        CredentialType::SocialSecurityCard,
        CredentialType::MedicareCard,
        CredentialType::EmployeeCard,
        CredentialType::PractisingCertificate,
        CredentialType::TitleCertificate,
        CredentialType::BusinessLicense,
        CredentialType::OrganizationCode,
        CredentialType::Other,
    ];
    for credential_type in variants {
        let credential = CredentialInfo {
            id: Id::from(7),
            r#type: credential_type,
            number: "NUMBER".into(),
            verified: None,
        };
        let encoded =
            CredentialInfoCodec::encode(Some(&credential)).expect("a credential should encode");
        let decoded = CredentialInfoCodec::decode(Some(&encoded))
            .expect("a supported credential should decode")
            .expect("nonblank input should produce a credential");
        assert_eq!(decoded.r#type, credential_type);
        assert_eq!(decoded.number, "NUMBER");
        assert_eq!(decoded.id, Id::default());
    }
    assert_eq!(CredentialInfoCodec::encode(None), None);
    assert_eq!(CredentialInfoCodec::decode(None).unwrap(), None);
    assert_eq!(CredentialInfoCodec::decode(Some(" \t ")).unwrap(), None);
    assert!(matches!(
        CredentialInfoCodec::decode(Some("NO_SEPARATOR")),
        Err(CredentialInfoCodecError::InvalidFormat)
    ));
    assert!(matches!(
        CredentialInfoCodec::decode(Some("UNKNOWN-NUMBER")),
        Err(CredentialInfoCodecError::UnsupportedType(name)) if name == "UNKNOWN"
    ));
}

/// Verifies common code, mapping, FAQ, and schedule values normalize their
/// public fields and expose empty representations through both APIs.
#[test]
fn test_common_values_normalize_and_serialize_complete_data() {
    let now = Utc::now();
    let mut code = Code {
        app: Some(StatefulInfo::default()),
        standard: Some("  ICD  ".into()),
        code: "  A01  ".into(),
    };
    exercise_traits(&mut code);
    assert_eq!(code.standard.as_deref(), Some("ICD"));
    assert_eq!(code.code, "A01");
    assert!(!code.is_empty());
    assert!(serde_json::to_value(&code).unwrap().get("app").is_some());

    let mut map = CodeMap {
        id: Id::from(1),
        entity: "  DIAGNOSIS  ".into(),
        source: Some(code.clone()),
        platform_code: "  PLATFORM-A01  ".into(),
        create_time: Some(now),
        modify_time: Some(now),
        delete_time: Some(now),
    };
    exercise_traits(&mut map);
    assert_eq!(map.entity, "DIAGNOSIS");
    assert_eq!(map.platform_code, "PLATFORM-A01");
    assert!(!map.is_empty());
    let map_json = serde_json::to_value(&map).expect("a code map should serialize");
    for field in ["id", "source", "create_time", "modify_time", "delete_time"] {
        assert!(map_json.get(field).is_some(), "{field} must serialize");
    }

    let mut faq = Faq {
        id: Id::from(2),
        app: Some(StatefulInfo::default()),
        category: Some(InfoWithEntity::default()),
        product: Some(Info::default()),
        question: "  What is covered?  ".into(),
        answer: "  The plan details coverage.  ".into(),
        frequency: 3,
        state: State::Disabled,
        create_time: Some(now),
        modify_time: Some(now),
        delete_time: Some(now),
    };
    exercise_traits(&mut faq);
    assert_eq!(faq.question, "What is covered?");
    assert_eq!(faq.answer, "The plan details coverage.");
    assert!(!faq.is_empty());
    assert!(serde_json::to_value(&faq).unwrap().get("product").is_some());

    macro_rules! assert_faq_field_makes_value_nonempty {
        ($update:expr) => {{
            let mut value = Faq::default();
            $update(&mut value);
            assert!(!value.is_empty());
        }};
    }
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.id = Id::from(1));
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| {
        value.app = Some(StatefulInfo::default())
    });
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| {
        value.category = Some(InfoWithEntity::default())
    });
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| {
        value.product = Some(Info::default())
    });
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.question = "Q".into());
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.answer = "A".into());
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.frequency = 1);
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.state = State::Disabled);
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.create_time = Some(now));
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.modify_time = Some(now));
    assert_faq_field_makes_value_nonempty!(|value: &mut Faq| value.delete_time = Some(now));

    let mut schedule = Schedule {
        start_time: Some(now),
        end_time: Some(now),
        crontabs: Some(vec![" 0 0 * * * ".into(), "   ".into()]),
    };
    exercise_traits(&mut schedule);
    assert_eq!(
        schedule.crontabs,
        Some(vec!["0 0 * * *".into(), String::new()])
    );
    assert!(!schedule.is_empty());
    let schedule_json =
        serde_json::to_value(&schedule).expect("a schedule should serialize complete fields");
    for field in ["start_time", "end_time", "crontabs"] {
        assert!(schedule_json.get(field).is_some(), "{field} must serialize");
    }
}

/// Verifies composed information types update their optional application and
/// token through their public mixin traits.
#[test]
fn test_composed_information_mixins_mutate_and_project_public_values() {
    let app = StatefulInfo {
        id: Id::from(1),
        code: "APP".into(),
        name: "Application".into(),
        state: Some(State::Normal),
        delete_time: None,
    };
    let mut app_entity = InfoWithAppEntity::new(
        InfoWithEntity::new(
            Some(2),
            "ENTITY".into(),
            "Entity".into(),
            None,
            "claim".into(),
        ),
        None,
    );
    assert!(!app_entity.is_complete());
    app_entity.set_app(Some(app.clone()));
    assert_eq!(app_entity.app(), Some(&app));
    assert!(app_entity.is_complete());
    app_entity.set_app(None);
    assert_eq!(app_entity.app(), None);

    let token = Token {
        value: "secret".into(),
        ..Default::default()
    };
    let mut info_with_token =
        InfoWithToken::new(Info::new(Some(3), "USER".into(), "User".into(), None), None);
    assert!(!info_with_token.is_complete());
    info_with_token.set_token(Some(token.clone()));
    assert_eq!(info_with_token.token(), Some(&token));
    assert!(info_with_token.is_complete());
    info_with_token.set_token(None);
    assert_eq!(info_with_token.token(), None);

    let mut projection = Projection {
        info: app,
        token: Some(token.clone()),
    };
    assert_eq!(projection.state(), Some(State::Normal));
    projection.set_state(Some(State::Disabled));
    projection.set_token(None);
    let projected = projection.stateful_info_with_token();
    assert_eq!(projected.info.state, Some(State::Disabled));
    assert_eq!(projected.token, None);
    projection.set_token(Some(token));
    assert!(projection.stateful_info_with_token().token.is_some());
}
