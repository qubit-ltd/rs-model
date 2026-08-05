//! Contact-method values.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::VerifyState;
use crate::contact::{Address, Phone};

/// Contact methods and their independent verification states.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct Contact {
    /// Optional landline number.
    pub phone: Option<Phone>,
    /// Verification state for `phone` when present.
    pub phone_verified: Option<VerifyState>,
    /// Optional mobile number.
    pub mobile: Option<Phone>,
    /// Verification state for `mobile` when present.
    pub mobile_verified: Option<VerifyState>,
    /// Optional ASCII email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub email: Option<String>,
    /// Verification state for `email` when present.
    pub email_verified: Option<VerifyState>,
    /// Optional ASCII URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional postal address.
    pub address: Option<Address>,
    /// Verification state for `address` when present.
    pub address_verified: Option<VerifyState>,
}
