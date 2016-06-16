#![crate_name = "nest"]


pub mod libc {
    extern crate libc;
    pub use self::libc::*;
}


mod error;
mod net;
mod sys;
mod config;

pub use config::NsConfig;
pub use error::NsError;
pub type NsResult<T> = std::result::Result<T, NsError>;


pub fn init(config: &NsConfig) {

    for i in 0..2 {
        // start worker

    }
}

pub fn run() {

}

pub fn version() -> String {
    "0.1.0".to_string()
}
