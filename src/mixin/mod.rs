//! Shared composite values and traits from the Java model mixin package.

mod stateful_info;
mod stateful_info_with_token;
mod traits;

pub use stateful_info::StatefulInfo;
pub use stateful_info_with_token::StatefulInfoWithToken;
pub use traits::{
    Expirable, HasStatefulInfo, Stateful, WithAddress, WithApp,
    WithCategory, WithContact, WithCredential, WithLocation, WithMobile, WithOrganization,
    WithOwner, WithPayloads, WithSource, WithToken,
};
