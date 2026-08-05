//! Role assignments and permission collections.

mod privileges;
mod role;
mod user_role;

pub use privileges::{Privileges, PrivilegesCodecError};
pub use role::Role;
pub use user_role::UserRole;
