// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Complete employee records.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_mixin::{Emptyful, InfoWithEntity, Normalizable};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{
    commons::{Category, Credential, CredentialInfo, State},
    contact::Phone,
    mixin::StatefulInfo,
    person::{Gender, Person, User},
    upload::Attachment,
};

use super::{Department, EmployeeInfo, Organization};

/// A complete employee record within an organization.
#[allow(clippy::duplicated_attributes)]
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(
    unique(name = "employee_code", fields(code), ignore_case(code)),
    unique(
        name = "employee_organization_internal_code",
        fields(organization, internal_code),
        ignore_case(internal_code)
    ),
    unique(name = "employee_organization_mobile", fields(organization, mobile)),
    unique(
        name = "employee_organization_email",
        fields(organization, email),
        ignore_case(email)
    )
)]
pub struct Employee {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Optional registered username.
    #[model(
        reference(target = User, target_field = username),
        index,
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional linked person identifier.
    #[model(reference(target = Person, target_field = id), index)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i64>,
    /// Globally unique employee code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Optional organization-local employee code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_code: Option<String>,
    /// Employee name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Employee gender.
    #[model(index)]
    pub gender: Gender,
    /// Optional birthday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<NaiveDate>,
    /// Optional identity credential.
    #[model(
        reference(target = Credential, target_field = info, must_exist = false),
        index,
        opaque
    )]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialInfo>,
    /// Optional employee category.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<InfoWithEntity>,
    /// Employer information.
    #[model(reference(target = Organization, target_field = info), index, opaque)]
    pub organization: StatefulInfo,
    /// Optional department information.
    #[model(reference(target = Department, target_field = info), index, opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<StatefulInfo>,
    /// Optional landline number.
    #[model(index)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Phone>,
    /// Organization-unique mobile number.
    #[model(index)]
    #[redact(nested)]
    pub mobile: Phone,
    /// Optional organization-unique email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional ASCII website URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional profile photograph.
    #[model(reference(target = Attachment, target_field = id, must_exist = false), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Attachment>,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional practising credential.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practising_certificate: Option<CredentialInfo>,
    /// Optional professional-title credential.
    #[model(reference(target = Credential, target_field = info, must_exist = false), opaque)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_certificate: Option<CredentialInfo>,
    /// Optional practising category.
    #[model(text(min_chars = 1, max_chars = 256))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practising_type: Option<String>,
    /// Optional practising scope.
    #[model(text(min_chars = 1, max_chars = 256))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practising_scope: Option<String>,
    /// Optional job title.
    #[model(index, text(min_chars = 1, max_chars = 256))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// Optional comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Lifecycle state.
    #[model(index)]
    pub state: State,
    /// Whether this is test data.
    #[model(index)]
    pub test: bool,
    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            id: None,
            username: None,
            person_id: None,
            code: String::new(),
            internal_code: None,
            name: String::new(),
            gender: Gender::Unknown,
            birthday: None,
            credential: None,
            category: None,
            organization: StatefulInfo::default(),
            department: None,
            phone: None,
            mobile: Phone::default(),
            email: None,
            url: None,
            photo: None,
            description: None,
            practising_certificate: None,
            title_certificate: None,
            practising_type: None,
            practising_scope: None,
            job_title: None,
            comment: None,
            state: State::Normal,
            test: false,
            create_time: None,
            modify_time: None,
            delete_time: None,
        }
    }
}

impl Employee {
    /// Projects this employee into its source information view.
    #[must_use]
    pub fn info(&self) -> EmployeeInfo {
        EmployeeInfo {
            id: self.id,
            code: self.code.clone(),
            internal_code: self.internal_code.clone(),
            username: self.username.clone(),
            name: self.name.clone(),
            gender: self.gender,
            birthday: self.birthday,
            credential: self.credential.clone(),
            mobile: self.mobile.clone(),
            organization: self.organization.clone(),
            department: self.department.clone(),
            photo: self.photo.clone(),
            state: self.state,
            test: self.test,
            delete_time: self.delete_time,
        }
    }

    /// Assigns fields carried by an employee information view.
    pub fn assign_info(&mut self, info: &EmployeeInfo) {
        self.id = info.id;
        self.code.clone_from(&info.code);
        self.internal_code.clone_from(&info.internal_code);
        self.username.clone_from(&info.username);
        self.name.clone_from(&info.name);
        self.gender = info.gender;
        self.birthday = info.birthday;
        self.credential.clone_from(&info.credential);
        self.mobile.clone_from(&info.mobile);
        self.organization.clone_from(&info.organization);
        self.department.clone_from(&info.department);
        self.photo.clone_from(&info.photo);
        self.state = info.state;
        self.test = info.test;
        self.delete_time = info.delete_time;
    }

    /// Returns whether every property has its source empty representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

impl Emptyful for Employee {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Employee {
    fn normalize(&mut self) {
        self.username.normalize();
        self.code.normalize();
        self.internal_code.normalize();
        self.name.normalize();
        self.phone.normalize();
        self.mobile.normalize();
        self.email.normalize();
        self.url.normalize();
        self.description.normalize();
        self.practising_type.normalize();
        self.practising_scope.normalize();
        self.job_title.normalize();
        self.comment.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
