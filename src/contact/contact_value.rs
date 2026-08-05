//! Contact-method values.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::commons::VerifyState;
use crate::contact::{Address, Phone};

/// Contact methods and their independent verification states.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Contact {
    /// Optional landline number.
    #[redact(nested)]
    pub phone: Option<Phone>,
    /// Verification state for `phone` when present.
    pub phone_verified: Option<VerifyState>,
    /// Optional mobile number.
    #[redact(nested)]
    pub mobile: Option<Phone>,
    /// Verification state for `mobile` when present.
    pub mobile_verified: Option<VerifyState>,
    /// Optional ASCII email address.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[redact(level = "secret")]
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

impl Contact {
    /// Creates contact details when at least one contact value is present.
    ///
    /// Returns `None` when all five contact values are absent. Verification
    /// states are initially absent, matching the Java factory method.
    #[must_use]
    pub fn create(
        phone: Option<Phone>,
        mobile: Option<Phone>,
        email: Option<String>,
        url: Option<String>,
        address: Option<Address>,
    ) -> Option<Self> {
        if phone.is_none()
            && mobile.is_none()
            && email.is_none()
            && url.is_none()
            && address.is_none()
        {
            None
        } else {
            Some(Self {
                phone,
                mobile,
                email,
                url,
                address,
                ..Self::default()
            })
        }
    }

    /// Resets each present contact value to the unverified state.
    pub fn set_verify_state(&mut self) {
        self.phone_verified = self.phone.as_ref().map(|_| VerifyState::None);
        self.mobile_verified = self.mobile.as_ref().map(|_| VerifyState::None);
        self.email_verified = self.email.as_ref().map(|_| VerifyState::None);
        self.address_verified = self.address.as_ref().map(|_| VerifyState::None);
    }

    /// Copies verification states for values that are unchanged from `other`.
    ///
    /// Absent values receive no state, and changed present values are reset to
    /// [`VerifyState::None`].
    pub fn copy_verify_state(&mut self, other: &Self) {
        self.phone_verified = copied_verify_state(&self.phone, &other.phone, other.phone_verified);
        self.mobile_verified =
            copied_verify_state(&self.mobile, &other.mobile, other.mobile_verified);
        self.email_verified = copied_verify_state(&self.email, &other.email, other.email_verified);
        self.address_verified =
            copied_verify_state(&self.address, &other.address, other.address_verified);
    }
}

/// Selects the verification state for one current and previous contact value.
fn copied_verify_state<T: PartialEq>(
    current: &Option<T>,
    previous: &Option<T>,
    previous_state: Option<VerifyState>,
) -> Option<VerifyState> {
    match current {
        None => None,
        Some(_) if current == previous => previous_state,
        Some(_) => Some(VerifyState::None),
    }
}
