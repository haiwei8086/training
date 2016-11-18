
pub mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}

 // mod win_form;
// mod winiocp;
mod socket;


fn main() {

    // win_form::run();
    // winiocp::run();
    // socket::run();



    println!("Hello, world!");
}
