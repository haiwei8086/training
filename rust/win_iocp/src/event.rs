#![allow(dead_code)]
#![allow(unused_imports)]


use std::{mem, ptr};
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

use winapi::um::ioapiset::CreateIoCompletionPort;


use super::{context, consts, ffi, sock_addr, ip, socket};


pub fn init(ctx: &mut context::Context) {

    let socket_fd = socket::create();
    let iocp = create_iocp();
}


pub fn create_iocp() -> HANDLE {

    let iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };

    if iocp == ptr::null_mut() {
        println!("Create IOCP failed!");
        return INVALID_HANDLE_VALUE;
    }
    println!("Created IOCP successed. fd: {:?}", iocp);


    iocp
}


pub fn add_event() {

}
