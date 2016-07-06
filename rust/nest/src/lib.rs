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
mod nest;

mod os;

use nest::Nest;

pub use config::NsConfig;
pub use error::NsError;
pub type NsResult<T> = std::result::Result<T, NsError>;

pub fn new(config: &mut NsConfig) -> Nest
{
    println!("{:?}", os::consts::AF_INET6);
    Nest::new(config)
}

pub fn version() -> String
{
    "0.1.0".to_string()
}
