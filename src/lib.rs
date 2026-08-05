// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Domain objects migrated to the Qubit Rust platform.
//!
//! This crate begins with the shared dependencies and error boundary used by
//! the domain models migrated from Java.

pub mod error;
pub mod feedback;
pub mod file;
pub mod commons;
pub mod contact;
pub mod device;
pub mod mixin;
pub mod medical;
pub mod metadata;
pub mod organization;
pub mod notification;
pub mod order;
pub mod payment;
pub mod person;
pub mod privilege;
pub mod product;
pub mod service;
pub mod security;
pub mod setting;
pub mod shipping;
pub mod settlement;
pub mod task;
pub mod thirdpart;
pub mod audit;
pub mod activity;
pub mod address;
pub mod ai;
pub mod appointment;
pub mod claim;
pub mod china;
pub mod invoice;
pub mod system;
pub mod statistics;
pub mod upload;
pub mod util;
pub mod work;
mod entity;
mod field;
mod module;
mod operation;

pub use commons::*;
pub use entity::Entity;
pub use field::Field;
pub use error::{ModelError, ValidationViolation};
pub use module::Module;
pub use operation::Operation;
