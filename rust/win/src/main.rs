
pub mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}

pub mod kernel32 {
    extern crate kernel32;
    pub use self::kernel32::*;
}

pub mod ws2_32 {
    extern crate ws2_32;
    pub use self::ws2_32::*;
}

 // mod win_form;
// mod winiocp;
mod socket;

fn main() {

    // win_form::run();
    // winiocp::run();
    socket::run();
}
