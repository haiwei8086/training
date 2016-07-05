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


pub struct Nest {
    filters: Vec<T>
}


impl Nest {

    pub fn new() -> Nest {
        Nest {
            filters: Vec::new()
        }
    }

    pub fn use(&self) -> &Nest{

    }

    pub fn listen(&self) -> NsResult<usize> {

    }
}


pub fn new() -> Nest {
    return Nest::new()
}




pub fn version() -> String {
    "0.1.0".to_string()
}
