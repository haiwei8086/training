#![allow(dead_code)]
#![allow(unused_imports)]

use std::{mem};

use winapi::ctypes::{c_char, c_int};
use winapi::um::winsock2::{u_long, SOCKET, FIONBIO, SOL_SOCKET, SO_REUSEADDR, SO_RCVBUF};
use winapi::shared::ws2def::IPPROTO_IPV6;
use winapi::shared::ws2ipdef::IPV6_V6ONLY;

use winapi::um::winsock2::{ioctlsocket, setsockopt, getsockopt};



// not iocp
pub fn non_blocking(s: &SOCKET) -> i32 {
    let mut nb: u_long = 1;

    let ret = unsafe { ioctlsocket(*s, FIONBIO, &mut nb) };
    if ret == -1 {
        println!("[non_blocking] ioctlsocket failed.");
    }

    ret
}


pub fn blocking(s: &SOCKET) -> i32 {
    let mut nb = 0;

    let ret = unsafe { ioctlsocket(*s, FIONBIO, &mut nb) };
    if ret == -1 {
        println!("[blocking] ioctlsocket failed.");
    }

    ret
}


pub fn reuse_addr(s: &SOCKET) -> i32 {
    let mut reuse: c_char = 1;

    let ret = unsafe { 
        setsockopt(*s, SOL_SOCKET, SO_REUSEADDR, &mut reuse as *const c_char, mem::size_of::<c_int>() as c_int)
    };
    if ret == -1 {
        println!("[reuse_addr] setsockopt(SO_REUSEADDR) failed.");

        // Close socket
    }

    ret
}

pub fn ipv6_only(s: &SOCKET) -> i32 {
    let mut val: c_char = 1;

    let ret = unsafe { 
        setsockopt(*s, IPPROTO_IPV6 as c_int, IPV6_V6ONLY, &mut val as *const c_char, mem::size_of::<c_int>() as c_int)
    };
    if ret == -1 {
        println!("[ipv6_only] setsockopt(IPV6_V6ONLY) failed.");

        // Close socket
    }

    ret
}

/*
    Dynamic send buffering for TCP was added on Windows 7 and Windows Server 2008 R2. 
    By default, dynamic send buffering for TCP is enabled unless an application sets 
    the SO_SNDBUF socket option on the stream socket.
*/

// SO_RCVBUF: after listen
pub fn get_rcvbuf(s: &SOCKET) {
    let mut val: c_char = 0;
    let mut val_len: i32 = 0;

    let ret = unsafe { 
        getsockopt(*s, SOL_SOCKET, SO_RCVBUF, &mut val, &mut val_len) 
    };
    if ret == -1 {
        println!("[get_rcvbuf] getsockopt(SO_RCVBUF) failed.");

        // Close socket
    }
}


// SO_KEEPALIVE: after listen
// ?TCP_NODELAY: after listen