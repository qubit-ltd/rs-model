//! Person, user, and demographic domain models.

mod enums;
mod person_info;
mod user;
mod user_info;

pub use enums::{
    Blood, Education, Ethnic, Gender, Incoming, Industry, JobTitle, Marriage, Politics, Religion,
    SexOrientation, SocialNetwork,
};
pub use user::User;
pub use user_info::UserInfo;
pub use person_info::PersonInfo;
