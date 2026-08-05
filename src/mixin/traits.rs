// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;

use crate::{
    commons::{CredentialInfo, Owner, Payload, State, Token},
    contact::{Address, Contact, Location, Phone},
};

use super::StatefulInfo;

/// Provides a lifecycle state.
pub trait Stateful {
    /// Returns the current state.
    fn state(&self) -> Option<State>;

    /// Replaces the current state.
    fn set_state(&mut self, state: Option<State>);
}

/// Provides an optional expiration timestamp.
pub trait Expirable {
    /// Returns the expiration timestamp.
    fn expired(&self) -> Option<DateTime<Utc>>;

    /// Replaces the expiration timestamp.
    fn set_expired(&mut self, expired: Option<DateTime<Utc>>);
}

/// Exposes stateful identifying information.
pub trait HasStatefulInfo: Stateful {
    /// Returns the stateful information projection.
    fn stateful_info(&self) -> StatefulInfo;
}

/// Provides an optional token.
pub trait WithToken {
    /// Returns the token.
    fn token(&self) -> Option<&Token>;

    /// Replaces the token.
    fn set_token(&mut self, token: Option<Token>);
}

/// Provides a mobile phone number.
pub trait WithMobile {
    /// Returns the mobile phone.
    fn mobile(&self) -> Option<&Phone>;

    /// Replaces the mobile phone.
    fn set_mobile(&mut self, mobile: Option<Phone>);
}

/// Provides contact details.
pub trait WithContact {
    /// Returns the contact details.
    fn contact(&self) -> Option<&Contact>;

    /// Replaces the contact details.
    fn set_contact(&mut self, contact: Option<Contact>);
}

/// Provides a geographic location.
pub trait WithLocation {
    /// Returns the location.
    fn location(&self) -> Option<&Location>;

    /// Replaces the location.
    fn set_location(&mut self, location: Option<Location>);
}

/// Provides a postal address.
pub trait WithAddress {
    /// Returns the address.
    fn address(&self) -> Option<&Address>;

    /// Replaces the address.
    fn set_address(&mut self, address: Option<Address>);
}

/// Provides an owner.
pub trait WithOwner {
    /// Returns the owner.
    fn owner(&self) -> Option<&Owner>;

    /// Replaces the owner.
    fn set_owner(&mut self, owner: Option<Owner>);
}

/// Provides category information.
pub trait WithCategory {
    /// Returns the category.
    fn category(&self) -> Option<&InfoWithEntity>;

    /// Replaces the category.
    fn set_category(&mut self, category: Option<InfoWithEntity>);
}

/// Provides source information.
pub trait WithSource {
    /// Returns the source.
    fn source(&self) -> Option<&InfoWithEntity>;

    /// Replaces the source.
    fn set_source(&mut self, source: Option<InfoWithEntity>);
}

/// Provides an owning application.
pub trait WithApp {
    /// Returns the application information.
    fn app(&self) -> Option<&StatefulInfo>;

    /// Replaces the application information.
    fn set_app(&mut self, app: Option<StatefulInfo>);
}

/// Provides an owning organization.
pub trait WithOrganization {
    /// Returns the organization information.
    fn organization(&self) -> Option<&StatefulInfo>;

    /// Replaces the organization information.
    fn set_organization(&mut self, organization: Option<StatefulInfo>);
}

/// Provides credential information.
pub trait WithCredential {
    /// Returns the credential information.
    fn credential(&self) -> Option<&CredentialInfo>;

    /// Replaces the credential information.
    fn set_credential(&mut self, credential: Option<CredentialInfo>);
}

/// Provides payloads.
pub trait WithPayloads {
    /// Returns the payloads.
    fn payloads(&self) -> Option<&[Payload]>;

    /// Replaces the payloads.
    fn set_payloads(&mut self, payloads: Option<Vec<Payload>>);
}
