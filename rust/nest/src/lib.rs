#![crate_name = "nest"]


pub mod libc {
    extern crate libc;
    pub use self::libc::*;
}

pub mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}


mod error;
mod config;
mod core;
mod os;
mod traits;


pub use self::config::NsConfig;
pub use self::error::NsError;
pub use self::core::Nest;

pub type NsResult<T> = std::result::Result<T, NsError>;


pub fn new(config: &mut NsConfig) -> Nest
{
    Nest::new(config)
}

pub fn version() -> String
{
    "0.1.0".to_string()
}
