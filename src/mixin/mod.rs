//! Shared composite values and traits from the Java model mixin package.

mod info_with_app_entity;
mod info_with_token;
mod stateful_info;
mod stateful_info_with_token;
mod traits;
mod with_attachment;
mod with_attachments;
mod with_creator;
mod with_deleter;
mod with_modifier;
mod with_stateful_info_with_token;

pub use info_with_app_entity::InfoWithAppEntity;
pub use info_with_token::InfoWithToken;
pub use stateful_info::StatefulInfo;
pub use stateful_info_with_token::StatefulInfoWithToken;
pub use traits::{
    Expirable, HasStatefulInfo, Stateful, WithAddress, WithApp,
    WithCategory, WithContact, WithCredential, WithLocation, WithMobile, WithOrganization,
    WithOwner, WithPayloads, WithSource, WithToken,
};
pub use with_attachment::WithAttachment;
pub use with_attachments::WithAttachments;
pub use with_creator::WithCreator;
pub use with_deleter::WithDeleter;
pub use with_modifier::WithModifier;
pub use with_stateful_info_with_token::WithStatefulInfoWithToken;
