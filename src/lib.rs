// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared domain models and model-supporting types for the Qubit platform.
//!
//! The crate exposes migrated domain modules together with common identifiers,
//! authorization classifications, validation errors, and REST response helpers.

/// Activity and promotion domain models.
pub mod activity;
/// Address matching and validation domain models.
pub mod address;
/// AI-related domain models.
pub mod ai;
/// Appointment domain models.
pub mod appointment;
/// Audit-trail domain models.
pub mod audit;
/// China-specific reference data and domain models.
pub mod china;
/// Insurance-claim domain models.
pub mod claim;
/// Reusable base domain models.
pub mod commons;
/// Contact and geographic domain models.
pub mod contact;
/// Controller-facing request and response models.
pub mod controller;
/// Client-device domain models.
pub mod device;
mod entity;
/// Model validation and conversion errors.
pub mod error;
/// User-feedback domain models.
pub mod feedback;
mod field;
/// File metadata and storage models.
pub mod file;
/// Invoice domain models.
pub mod invoice;
/// Medical and prescription domain models.
pub mod medical;
/// Metadata domain models.
pub mod metadata;
/// Reusable domain-model traits and composite references.
pub mod mixin;
mod module;
/// Notification domain models.
pub mod notification;
mod operation;
/// Ordering and fulfillment domain models.
pub mod order;
/// Organization and employment domain models.
pub mod organization;
/// Payment domain models.
pub mod payment;
/// Person and account domain models.
pub mod person;
/// Authorization and privilege domain models.
pub mod privilege;
/// Product-catalog domain models.
pub mod product;
/// Security credential and signature domain models.
pub mod security;
/// Service-delivery domain models.
pub mod service;
/// Platform-setting domain models.
pub mod setting;
/// Settlement and transaction domain models.
pub mod settlement;
/// Shipping domain models.
pub mod shipping;
/// Statistical reporting domain models.
pub mod statistics;
/// System-level domain models.
pub mod system;
/// Task-management domain models.
pub mod task;
/// Third-party integration domain models.
pub mod thirdpart;
/// File-upload domain models.
pub mod upload;
/// Small utility types used by model APIs.
pub mod util;
/// Work-scheduling domain models.
pub mod work;

/// Common base models re-exported at the crate root.
pub use commons::*;
/// Domain entity classification used by generic references.
pub use entity::Entity;
/// Error returned when model construction, conversion, or validation fails.
pub use error::ModelError;
/// A validation constraint failure; callers must redact sensitive rejected values.
pub use error::ValidationViolation;
/// Field classification used by validation and localized messages.
pub use field::Field;
/// Functional system module used to group operations.
pub use module::Module;
/// Independently authorizable system operation.
pub use operation::Operation;
