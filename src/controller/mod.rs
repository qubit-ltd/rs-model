// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Wire models for authentication, registration, binding, and audited queries.

mod app_authenticate_params;
mod auditable_query_params;
mod bind_device_params;
mod bind_employee_params;
mod bind_person_params;
mod login_params;
mod login_response;
mod null_sort_option;
mod register_user_params;
mod sort_order;
mod unupdatable_query_params;
mod update_password_params;

pub use app_authenticate_params::AppAuthenticateParams;
pub use auditable_query_params::AuditableQueryParams;
pub use bind_device_params::BindDeviceParams;
pub use bind_employee_params::BindEmployeeParams;
pub use bind_person_params::BindPersonParams;
pub use login_params::LoginParams;
pub use login_response::LoginResponse;
pub use null_sort_option::NullSortOption;
pub use register_user_params::RegisterUserParams;
pub use sort_order::SortOrder;
pub use unupdatable_query_params::UnupdatableQueryParams;
pub use update_password_params::UpdatePasswordParams;
