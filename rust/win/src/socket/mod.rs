#![allow(dead_code)]
use super::winapi;


mod consts;
mod ffi;
mod addr;
mod process;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AddressFamily {
    Unix = consts::AF_UNIX,
    Inet = consts::AF_INET,
    Inet6 = consts::AF_INET6,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SockType {
    Stream = consts::SOCK_STREAM,
    Datagram = consts::SOCK_DGRAM,
    SeqPacket = consts::SOCK_SEQPACKET,
    Raw = consts::SOCK_RAW,
    Rdm = consts::SOCK_RDM,
}

// use SetConsoleCtrlHandler
pub unsafe extern "system" fn console_handler(ctrl_type: u32) -> i32 {
    match ctrl_type {
        winapi::wincon::CTRL_C_EVENT => println!("Ctrl-C pressed, exiting"),
        winapi::wincon::CTRL_BREAK_EVENT => println!("Ctrl-Break pressed, exiting"),
        winapi::wincon::CTRL_CLOSE_EVENT => println!("console closing, exiting"),
        winapi::wincon::CTRL_LOGOFF_EVENT => println!("user logs off, exiting"),
        winapi::wincon::CTRL_SHUTDOWN_EVENT => println!("Ctrl-shutdown pressed, exiting"),
        _ => println!("Console Ctrl Handler: {}", ctrl_type),
    }

    return 1;
}


pub fn run() {
    process::Process::new();
}
