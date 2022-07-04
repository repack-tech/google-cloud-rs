mod client;
mod entity;
mod key;
mod query;
mod value;
mod api {
    pub mod r#type {
        include!("api/google.r#type.rs");
    }
    pub mod datastore {
        #![allow(clippy::enum_variant_names, clippy::large_enum_variant)]
        pub mod v1 {
            include!("api/google.datastore.v1.rs");
        }
    }
    pub use self::datastore::v1::*;
    pub use self::r#type::*;
}

pub use self::client::*;
pub use self::entity::*;
pub use self::key::*;
pub use self::query::*;
pub use self::value::*;

/// The error type for the Datastore module.
pub type Error = crate::error::Error;
