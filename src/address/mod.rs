//! Address-domain models, builders, mixins, and errors.

mod address_builder;
mod address_error;

pub use crate::contact::{Address, City, Contact, Country, District, Province, Region, Street};
pub use crate::mixin::{WithAddress, WithContact};
pub use address_builder::AddressBuilder;
pub use address_error::{AddressErrorCode, MismatchMobileException};
