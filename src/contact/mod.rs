//! Contact and geographic domain models.

mod address;
mod administrative_regions;
mod contact_value;
mod coordinate_system;
mod location;
mod phone;
mod region;

pub use address::Address;
pub use administrative_regions::{City, Country, District, Province, Street};
pub use contact_value::Contact;
pub use coordinate_system::CoordinateSystem;
pub use location::Location;
pub use phone::Phone;
pub use region::Region;
