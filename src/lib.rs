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

pub mod activity;
pub mod address;
pub mod ai;
pub mod appointment;
pub mod audit;
pub mod china;
pub mod claim;
pub mod commons;
pub mod contact;
pub mod controller;
pub mod device;
mod entity;
pub mod error;
pub mod feedback;
mod field;
pub mod file;
pub mod invoice;
pub mod medical;
pub mod metadata;
pub mod mixin;
mod module;
pub mod notification;
mod operation;
pub mod order;
pub mod organization;
pub mod payment;
pub mod person;
pub mod privilege;
pub mod product;
pub mod security;
pub mod service;
pub mod setting;
pub mod settlement;
pub mod shipping;
pub mod statistics;
pub mod system;
pub mod task;
pub mod thirdpart;
pub mod upload;
pub mod util;
pub mod work;

pub use commons::*;
pub use entity::Entity;
pub use error::{
    ModelError,
    ValidationViolation,
};
pub use field::Field;
pub use module::Module;
pub use operation::Operation;
