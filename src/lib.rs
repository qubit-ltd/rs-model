//! Domain objects migrated to the Qubit Rust platform.
//!
//! This crate begins with the shared dependencies and error boundary used by
//! the domain models migrated from Java.

pub mod error;
pub mod commons;
pub mod contact;
pub mod mixin;
pub mod medical;
pub mod organization;
pub mod person;
pub mod util;
mod entity;
mod module;
mod operation;

pub use commons::*;
pub use entity::Entity;
pub use error::{ModelError, ValidationViolation};
pub use module::Module;
pub use operation::Operation;
