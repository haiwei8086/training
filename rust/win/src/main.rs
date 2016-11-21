
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


extern "system" {
    #[link(name = "LPFN_ACCEPTEX")]
    pub fn LPFN_ACCEPTEX(sListenSocket: winapi::SOCKET,
    sAcceptSocket: winapi::SOCKET,
    lpOutputBuffer: winapi::PVOID,
    dwReceiveDataLength: winapi::DWORD,
    dwLocalAddressLength: winapi::DWORD,
    dwRemoteAddressLength: winapi::DWORD,
    lpdwBytesReceived: winapi::LPDWORD,
    lpOverlapped: winapi::LPOVERLAPPED) -> bool;
}

fn main() {

    // win_form::run();
    // winiocp::run();
    // socket::run();

    let fn_a: system::LPFN_ACCEPTEX = std::ptr::null();

    println!("fn: {:?}", fn_a);

}
