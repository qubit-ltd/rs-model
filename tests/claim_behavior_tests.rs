// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioural integration tests for individual and enterprise claim models.

use bigdecimal::BigDecimal;
use chrono::Utc;
use qubit_id::Id;

use qubit_mixin::Info;
use qubit_model::claim::AccidentReason;
use qubit_model::claim::InsuranceClaim;
use qubit_model::claim::InsuranceClaimAmount;
use qubit_model::claim::InsuranceClaimInvoice;
use qubit_model::claim::InsuranceClaimInvoiceStatus;
use qubit_model::claim::InsuranceClaimInvoiceType;
use qubit_model::claim::InsuranceClaimMedical;
use qubit_model::claim::InsuranceClaimStatus;
use qubit_model::claim::InsuranceClaimStatusGroup;
use qubit_model::claim::InsuredStatus;
use qubit_model::claim::QuickCompensationState;
use qubit_model::claim::enterprise::EnterpriseClaim;
use qubit_model::claim::enterprise::EnterpriseClaimInvoice;
use qubit_model::claim::enterprise::EnterpriseClaimItem;
use qubit_model::claim::enterprise::EnterpriseClaimItemStatus;
use qubit_model::claim::enterprise::EnterpriseClaimMedical;
use qubit_model::claim::enterprise::EnterpriseClaimSelfCareItem;
use qubit_model::claim::enterprise::EnterpriseClaimStatus;
use qubit_model::claim::enterprise::EnterpriseClaimStatusGroup;
use qubit_model::claim::enterprise::EnterpriseHistoryClaimAmount;
use qubit_model::claim::enterprise::EnterpriseInsuredType;
use qubit_model::claim::enterprise::EnterpriseOwnership;
use qubit_model::claim::enterprise::SaveStatus;
use qubit_model::commons::Currency;
use qubit_model::commons::DictEntryInfo;
use qubit_model::commons::Kinship;
use qubit_model::commons::State;
use qubit_model::mixin::StatefulInfo;
use qubit_model::order::Client;
use qubit_model::payment::Account;
use qubit_model::payment::AccountType;
use qubit_model::product::Product;
use qubit_model::product::Quality;
use qubit_model_metadata::TypeShape;
use qubit_model_metadata::metadata_of;

/// Builds a minimal client accepted by the public claim API.
fn client(name: &str) -> Client {
    Client {
        id: Id::default(),
        name: name.to_owned(),
        credential: None,
        gender: None,
        birthday: None,
        mobile: None,
        email: None,
        has_medicare: None,
        medicare_type: None,
        medicare_card: None,
        medicare_city: None,
        has_social_security: None,
        social_security_card: None,
        social_security_city: None,
        guardian: None,
        return_status: None,
        kinship: None,
        payload: None,
    }
}

/// Builds a minimal product accepted by the public claim API.
fn product() -> Product {
    let now = Utc::now();
    Product {
        id: Id::default(),
        code: "CLAIM_PRODUCT".into(),
        name: "Claim product".into(),
        app: StatefulInfo::default(),
        category: None,
        quality: Quality::BrandNew,
        currency: Currency::Cny,
        image: None,
        description: None,
        valid_from: None,
        valid_until: None,
        brand: None,
        origin: None,
        manufacturer: None,
        seller: Info::default(),
        sale_from: now.naive_utc(),
        sale_until: None,
        need_delivery: false,
        allow_return: false,
        allow_change: false,
        need_client: false,
        constraint: None,
        items: Vec::new(),
        state: State::Normal,
        create_time: now,
        modify_time: None,
        delete_time: None,
    }
}

/// Builds the account embedded in an individual claim.
fn account() -> Account {
    let now = Utc::now();
    Account {
        id: Id::default(),
        app: StatefulInfo::default(),
        owner_type: "PERSON".into(),
        owner_id: Id::from(1),
        r#type: AccountType::BankCard,
        name: "Claim account".into(),
        number: None,
        provider: None,
        create_time: now,
        modify_time: None,
        delete_time: None,
    }
}

/// Builds the amount aggregate embedded in an individual claim.
fn claim_amount() -> InsuranceClaimAmount {
    InsuranceClaimAmount {
        id: Id::default(),
        claim_id: Id::from(1),
        total_amount: None,
        medicare_amount: None,
        self_paid_amount: None,
        self_care_amount: None,
        fund_paid_amount: None,
        serious_illness_paid: None,
        serious_illness_insurance_paid: None,
        civil_affair_subsidy_paid: None,
        self_paid_claim_amount: None,
        self_care_claim_amount: None,
        total_claim_amount: None,
        actual_self_paid_amount: None,
        actual_self_care_amount: None,
        actual_paid_amount: None,
        paid_amount_calibration: false,
        pay_time: None,
        endcase_date: None,
        payload: None,
        create_time: Utc::now(),
        modify_time: None,
    }
}

/// Builds an individual claim with a selectable workflow state.
fn individual_claim(status: InsuranceClaimStatus) -> InsuranceClaim {
    let now = Utc::now();
    let status_group: Box<dyn Fn(InsuranceClaimStatus) -> InsuranceClaimStatusGroup> =
        Box::new(InsuranceClaimStatus::status_group);
    InsuranceClaim {
        id: Id::default(),
        product: product(),
        company: Info::default(),
        source: Info::default(),
        reason: AccidentReason::Disease,
        policy_number: "policy".into(),
        insured: client("Insured"),
        insured_address: None,
        insured_status: None,
        claimant_relation: Kinship::Self_,
        claimant: client("Claimant"),
        claimant_address: None,
        accident_date: now.date_naive(),
        accident_place: "Shanghai".into(),
        accident_description: "description".into(),
        hospital: None,
        treatment_start_date: None,
        treatment_end_date: None,
        quick_compensation_state: QuickCompensationState::Success,
        currency: Some(Currency::Cny),
        total_paid_amount: None,
        payee_name: "Payee".into(),
        account: account(),
        number: "CLAIM-1".into(),
        issue_time: None,
        cancel_time: None,
        complete_time: None,
        status,
        status_group: status_group(status),
        notes: String::new(),
        payload: None,
        attachment_list: Vec::new(),
        events: Vec::new(),
        medical_list: Vec::new(),
        saved_invoices: Vec::new(),
        amount: claim_amount(),
        create_time: now,
        modify_time: None,
        delete_time: None,
    }
}

/// Builds an enterprise claim with a selectable workflow state.
fn enterprise_claim(status: EnterpriseClaimStatus) -> EnterpriseClaim {
    let status_group: Box<dyn Fn(EnterpriseClaimStatus) -> EnterpriseClaimStatusGroup> =
        Box::new(EnterpriseClaimStatus::status_group);
    EnterpriseClaim {
        id: Id::default(),
        product: product(),
        reason: AccidentReason::Disease,
        insured_status: InsuredStatus::Recovery,
        insured: client("Insured"),
        claimant_relation: Kinship::Self_,
        claimant: client("Claimant"),
        issue_time: None,
        cancel_time: None,
        complete_time: None,
        status,
        status_group: status_group(status),
        notes: String::new(),
        quick_compensation_state: QuickCompensationState::Success,
        events: Vec::new(),
        attachment_list: Vec::new(),
        create_time: Utc::now(),
        modify_time: None,
    }
}

/// Verifies individual status grouping and operation permissions for every
/// source workflow branch.
#[test]
fn test_individual_claim_statuses_drive_operation_permissions() {
    let status_group: Box<dyn Fn(InsuranceClaimStatus) -> InsuranceClaimStatusGroup> =
        Box::new(InsuranceClaimStatus::status_group);
    let allow_client =
        std::hint::black_box(InsuranceClaim::allow_client_operation as fn(&InsuranceClaim) -> bool);
    let allow_reject =
        std::hint::black_box(InsuranceClaim::allow_system_reject as fn(&InsuranceClaim) -> bool);
    let allow_accept =
        std::hint::black_box(InsuranceClaim::allow_system_accept as fn(&InsuranceClaim) -> bool);
    let cases = [
        (InsuranceClaimStatus::NotSubmitted, true, false, false),
        (
            InsuranceClaimStatus::ClaimApplicationWaitAudit,
            false,
            true,
            false,
        ),
        (
            InsuranceClaimStatus::ClaimApplicationAudited,
            false,
            true,
            true,
        ),
        (InsuranceClaimStatus::TemporarySaved, false, true, true),
        (InsuranceClaimStatus::SystemAudited, false, false, false),
        (InsuranceClaimStatus::SystemRejected, true, false, false),
        (
            InsuranceClaimStatus::WaitInsuranceCompanyAudited,
            false,
            false,
            false,
        ),
        (
            InsuranceClaimStatus::InsuranceCompanyAccepted,
            false,
            false,
            false,
        ),
        (
            InsuranceClaimStatus::InsuranceCompanyRejected,
            true,
            false,
            false,
        ),
        (
            InsuranceClaimStatus::InsuranceCompanyCompleted,
            false,
            false,
            false,
        ),
        (
            InsuranceClaimStatus::InsuranceCompanyAnnulOrRefused,
            false,
            false,
            false,
        ),
        (InsuranceClaimStatus::Canceled, false, false, false),
    ];
    for (status, client_allowed, reject_allowed, accept_allowed) in cases {
        let claim = individual_claim(status);
        assert_eq!(claim.status_group, status_group(status));
        assert_eq!(allow_client(&claim), client_allowed);
        assert_eq!(allow_reject(&claim), reject_allowed);
        assert_eq!(allow_accept(&claim), accept_allowed);
        let json = serde_json::to_value(&claim).expect("an individual claim should serialize");
        assert_eq!(json["status"], serde_json::to_value(status).unwrap());
    }
    let unfinished = std::hint::black_box(
        InsuranceClaimStatus::list_not_finished_status as fn() -> &'static [InsuranceClaimStatus],
    );
    assert_eq!(unfinished().len(), 8);
}

/// Verifies enterprise status grouping and permissions for every workflow
/// branch in the public model.
#[test]
fn test_enterprise_claim_statuses_drive_operation_permissions() {
    let status_group: Box<dyn Fn(EnterpriseClaimStatus) -> EnterpriseClaimStatusGroup> =
        Box::new(EnterpriseClaimStatus::status_group);
    let allow_client: Box<dyn Fn(&EnterpriseClaim) -> bool> =
        Box::new(EnterpriseClaim::allow_client_operation);
    let allow_reject: Box<dyn Fn(&EnterpriseClaim) -> bool> =
        Box::new(EnterpriseClaim::allow_reject);
    let allow_admin: Box<dyn Fn(&EnterpriseClaim) -> bool> =
        Box::new(EnterpriseClaim::allow_admin_operation);
    let cases = [
        (EnterpriseClaimStatus::NotSubmitted, true, false, false),
        (
            EnterpriseClaimStatus::ClaimApplicationWaitAudit,
            false,
            true,
            false,
        ),
        (EnterpriseClaimStatus::SystemRejected, true, false, false),
        (
            EnterpriseClaimStatus::ClaimApplicationAudited,
            false,
            true,
            true,
        ),
        (EnterpriseClaimStatus::TemporarySaved, false, true, true),
        (
            EnterpriseClaimStatus::WaitInsuranceCompanyAudited,
            false,
            false,
            false,
        ),
        (
            EnterpriseClaimStatus::InsuranceCompanyCompleted,
            false,
            false,
            false,
        ),
        (EnterpriseClaimStatus::Canceled, false, false, false),
    ];
    for (status, client_allowed, reject_allowed, admin_allowed) in cases {
        let claim = enterprise_claim(status);
        assert_eq!(claim.status_group, status_group(status));
        assert_eq!(allow_client(&claim), client_allowed);
        assert_eq!(allow_reject(&claim), reject_allowed);
        assert_eq!(allow_admin(&claim), admin_allowed);
        let json = serde_json::to_value(&claim).expect("an enterprise claim should serialize");
        assert_eq!(json["status"], serde_json::to_value(status).unwrap());
    }
    let unfinished = std::hint::black_box(
        EnterpriseClaimStatus::list_not_finished_status as fn() -> &'static [EnterpriseClaimStatus],
    );
    assert_eq!(unfinished().len(), 6);
}

/// Verifies invoice amount and self-care validation accepts and rejects the
/// source-domain boundary cases.
#[test]
fn test_claim_invoice_validations_check_component_boundaries() {
    let now = Utc::now();
    let self_care_item = EnterpriseClaimSelfCareItem {
        id: Id::default(),
        claim_invoice_id: Id::from(1),
        name: "Class B medicine".into(),
        medicare_charge_code: "B001".into(),
        amount: BigDecimal::from(10),
        ratio: 1.0,
        create_time: now,
        delete_time: None,
    };
    let mut enterprise = EnterpriseClaimInvoice {
        id: Id::default(),
        claim_id: Id::from(1),
        claim_medical_id: Id::from(1),
        attachment_id: Id::from(1),
        number: "E-1".into(),
        deductible: BigDecimal::from(0),
        amount: BigDecimal::from(60),
        self_paid_amount: BigDecimal::from(10),
        self_care_amount: BigDecimal::from(10),
        fund_paid_amount: BigDecimal::from(10),
        serious_illness_amount: BigDecimal::from(10),
        serious_illness_insurance_amount: BigDecimal::from(10),
        no_reimbursement_amount: BigDecimal::from(0),
        invalid_amount: BigDecimal::from(0),
        class_b_self_care_amount: BigDecimal::from(0),
        self_amount: BigDecimal::from(0),
        civil_affair_subsidy_amount: BigDecimal::from(10),
        medicare_amount: BigDecimal::from(0),
        source: Default::default(),
        operator_name: None,
        status: SaveStatus::Saved,
        accuracy: true,
        inaccurate_reason: String::new(),
        self_care_items: vec![self_care_item],
        claim_base: BigDecimal::from(0),
        claim_amount: BigDecimal::from(0),
        create_time: now,
        modify_time: None,
        delete_time: None,
    };
    assert!(enterprise.check_amount());
    assert!(enterprise.check_self_care_items());
    enterprise.amount = BigDecimal::from(59);
    assert!(!enterprise.check_amount());
    enterprise.self_care_items[0].ratio = 1.1;
    assert!(!enterprise.check_self_care_items());

    let mut individual = InsuranceClaimInvoice {
        id: Id::default(),
        claim_id: Id::from(1),
        claim_medical_id: Id::from(1),
        attachment_id: Id::from(1),
        number: "I-1".into(),
        amount: BigDecimal::from(30),
        fund_paid_amount: BigDecimal::from(10),
        self_paid_amount: BigDecimal::from(10),
        self_care_amount: BigDecimal::from(10),
        medicare_amount: BigDecimal::from(0),
        serious_illness_paid: None,
        serious_illness_insurance_paid: None,
        civil_affair_subsidy_paid: None,
        self_amount: None,
        past_symptom: false,
        r#type: InsuranceClaimInvoiceType::Hospitalization,
        status: InsuranceClaimInvoiceStatus::Saved,
        accuracy: true,
        inaccurate_reason: String::new(),
        self_amount_supply: None,
        create_time: now,
        modify_time: None,
        delete_time: None,
        costs: Vec::new(),
    };
    assert!(individual.check_amount());
    individual.amount = BigDecimal::from(29);
    assert!(!individual.check_amount());
}

/// Verifies an enterprise calculation item derives its hospital and disease
/// summaries for empty, uniform, and mixed medical encounter collections.
#[test]
fn test_enterprise_claim_item_initializes_hospital_and_disease_summaries() {
    let now = Utc::now();
    let medical =
        |hospital_name: &str, hospital_level: i32, disease_code: &str| EnterpriseClaimMedical {
            id: Id::default(),
            claim_id: Id::from(1),
            treatment_start_date: now.date_naive(),
            treatment_end_date: now.date_naive(),
            number: None,
            claim_apply_id: None,
            medical_category: None,
            disease: Some(DictEntryInfo {
                id: Id::default(),
                code: disease_code.into(),
                name: String::new(),
                dict_id: Id::default(),
                params: Vec::new(),
                delete_time: None,
            }),
            hospital: Some(DictEntryInfo {
                id: Id::default(),
                code: String::new(),
                name: hospital_name.into(),
                dict_id: Id::default(),
                params: Vec::new(),
                delete_time: None,
            }),
            hospital_level: Some(hospital_level),
            operator_name: None,
            insured_type: EnterpriseInsuredType::InService,
            status: SaveStatus::Saved,
            invoices: Vec::new(),
            create_time: now,
            modify_time: None,
            delete_time: None,
        };
    let history = EnterpriseHistoryClaimAmount {
        id: Id::default(),
        product_id: Id::from(1),
        name: "Insured".into(),
        credential_number: "credential".into(),
        medical_category: DictEntryInfo {
            id: Id::default(),
            code: "GENERAL".into(),
            name: "General".into(),
            dict_id: Id::default(),
            params: Vec::new(),
            delete_time: None,
        },
        claim_base: BigDecimal::from(0),
        deductible: BigDecimal::from(0),
        overall_fund_amount: BigDecimal::from(0),
        create_time: now,
        modify_time: None,
    };
    let mut item = EnterpriseClaimItem {
        id: Id::default(),
        claim_id: Id::from(1),
        medical_category: history.medical_category.clone(),
        insured_type: None,
        amount: BigDecimal::from(0),
        overall_fund_amount: BigDecimal::from(0),
        invalid_amount: BigDecimal::from(0),
        deductible: BigDecimal::from(0),
        self_amount: BigDecimal::from(0),
        claim_base: BigDecimal::from(0),
        claim_amount: BigDecimal::from(0),
        actual_claim_amount: BigDecimal::from(0),
        over_upper_limit: BigDecimal::from(0),
        serious_illness_amount: BigDecimal::from(0),
        serious_illness_insurance_amount: BigDecimal::from(0),
        yangzi_supply: BigDecimal::from(0),
        hospital_name: None,
        hospital_level: None,
        disease_code: None,
        actual_paid_amount: BigDecimal::from(0),
        paid_date: None,
        endcase_date: None,
        operator_name: None,
        description: None,
        status: EnterpriseClaimItemStatus::Created,
        medicals: Vec::new(),
        history_claim_amount: history,
        deduct_deductible: false,
        create_time: now,
        modify_time: None,
        delete_time: None,
    };
    item.init_hospital_and_disease();
    assert_eq!(item.hospital_name, None);
    assert_eq!(item.disease_code, None);

    item.medicals = vec![medical("Central Hospital", 2, "A01")];
    item.init_hospital_and_disease();
    assert_eq!(item.hospital_name.as_deref(), Some("Central Hospital"));
    assert_eq!(item.hospital_level, Some(2));
    assert_eq!(item.disease_code.as_deref(), Some("A01"));

    item.medicals.push(medical("East Hospital", 3, "B02"));
    item.init_hospital_and_disease();
    assert_eq!(item.hospital_name.as_deref(), Some("其他"));
    assert_eq!(item.hospital_level, Some(3));
    assert_eq!(item.disease_code.as_deref(), Some("A01"));
}

/// Verifies every claim leaf enumeration keeps its explicit wire spelling.
#[test]
fn test_claim_leaf_enumerations_round_trip_through_json() {
    let values = [
        serde_json::to_value(AccidentReason::Birth).unwrap(),
        serde_json::to_value(InsuredStatus::UnderTreatment).unwrap(),
        serde_json::to_value(QuickCompensationState::Fetching).unwrap(),
        serde_json::to_value(InsuranceClaimInvoiceStatus::IgnoredGt).unwrap(),
        serde_json::to_value(InsuranceClaimInvoiceType::ClinicSpecial).unwrap(),
        serde_json::to_value(InsuranceClaimStatusGroup::AuditRejection).unwrap(),
        serde_json::to_value(EnterpriseClaimStatusGroup::Register).unwrap(),
        serde_json::to_value(SaveStatus::NotSaved).unwrap(),
    ];
    assert_eq!(
        values,
        [
            serde_json::Value::String("BIRTH".into()),
            serde_json::Value::String("UNDER_TREATMENT".into()),
            serde_json::Value::String("FETCHING".into()),
            serde_json::Value::String("IGNORED_GT".into()),
            serde_json::Value::String("CLINIC_SPECIAL".into()),
            serde_json::Value::String("AUDIT_REJECTION".into()),
            serde_json::Value::String("REGISTER".into()),
            serde_json::Value::String("NOT_SAVED".into()),
        ]
    );

    let insured_type_cases = [
        (EnterpriseInsuredType::InService, "10", "在职"),
        (EnterpriseInsuredType::Retired, "11", "退休"),
        (EnterpriseInsuredType::Resigned, "12", "退职"),
        (EnterpriseInsuredType::OverSeventy, "13", "70岁以上"),
        (EnterpriseInsuredType::OnlyChild, "31", "独生子女<=16"),
        (EnterpriseInsuredType::ChildDonorGenus, "32", "子女供属"),
        (EnterpriseInsuredType::DonorGenus, "41", "供属"),
    ];
    let insured_type_code: Box<dyn Fn(EnterpriseInsuredType) -> &'static str> =
        Box::new(EnterpriseInsuredType::code);
    let insured_type_description: Box<dyn Fn(EnterpriseInsuredType) -> &'static str> =
        Box::new(EnterpriseInsuredType::description);
    for (insured_type, code, description) in insured_type_cases {
        assert_eq!(insured_type_code(insured_type), code);
        assert_eq!(insured_type_description(insured_type), description);
    }

    let ownership_cases = [
        (EnterpriseOwnership::Yangtze, "1", "扬子"),
        (EnterpriseOwnership::Reform, "0", "改制"),
        (EnterpriseOwnership::CoSolution, "2", "协解"),
        (EnterpriseOwnership::Test, "z", "测试"),
    ];
    let ownership_code =
        std::hint::black_box(EnterpriseOwnership::code as fn(EnterpriseOwnership) -> &'static str);
    let ownership_description = std::hint::black_box(
        EnterpriseOwnership::description as fn(EnterpriseOwnership) -> &'static str,
    );
    for (ownership, code, description) in ownership_cases {
        let ownership = std::hint::black_box(ownership);
        assert_eq!(ownership_code(ownership), code);
        assert_eq!(ownership_description(ownership), description);
    }
}

/// Verifies claim metadata keeps the unsupported tuple payload opaque without
/// treating an unannotated medical amount as money.
#[test]
fn test_claim_metadata_preserves_source_constraints() {
    let claim = metadata_of::<InsuranceClaim>();
    let payload = claim.field("payload").expect("claim payload field");
    assert!(matches!(payload.field_type().shape(), TypeShape::Opaque));

    let medical = metadata_of::<InsuranceClaimMedical>();
    assert!(
        medical
            .field("amount")
            .expect("claim medical amount field")
            .decimal_constraint()
            .is_none()
    );
}
