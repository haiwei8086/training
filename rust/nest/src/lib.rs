#![crate_name = "nest"]

pub mod libc {
    extern crate libc;
    pub use self::libc::*;
}

pub mod error;
pub mod net;

type NsResult<T> = std::result::Result<T, error::NsError>;


pub fn version() -> String {
    "0.1.0".to_string()
}
