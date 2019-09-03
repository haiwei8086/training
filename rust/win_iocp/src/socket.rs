#![allow(dead_code)]
#![allow(unused_imports)]

use std::{mem, ptr};

use winapi::ctypes::{c_char, c_int};
use winapi::um::winsock2::{u_long, SOCKET, FIONBIO, SOL_SOCKET, SO_REUSEADDR, SO_RCVBUF, WSA_FLAG_OVERLAPPED, INVALID_SOCKET, SOMAXCONN};
use winapi::shared::ws2def::{AF_INET, SOCK_STREAM, IPPROTO_IPV6, SOCKADDR, SOCKADDR_IN};
use winapi::shared::ws2ipdef::IPV6_V6ONLY;

use winapi::um::winsock2::{WSASocketW, ioctlsocket, setsockopt, getsockopt, bind, listen};


use super::{consts, ffi, sock_addr, ip, socket};



pub fn create() -> usize {
    let addr = sock_addr::InetAddr::new(ip::IPAddr::V4(<ip::IPAddrV4>::new(127, 0, 0, 1)), 8080);
    let addr_len = mem::size_of::<ffi::sockaddr_in>() as ffi::socklen_t;


    let sock_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };
    if INVALID_SOCKET == sock_fd {
        println!("Invalid socket.");
        return 0;
    }

    println!("Created socket. fd: {:?}", sock_fd);

    let s_addr = match addr {
        sock_addr::InetAddr::V4(ref addr) => unsafe { mem::transmute::<&ffi::sockaddr_in, *const SOCKADDR>(&addr.0) },
        sock_addr::InetAddr::V6(ref addr) => unsafe { mem::transmute::<&ffi::sockaddr_in6, *const SOCKADDR>(&addr.0) },
    };

    if -1 == unsafe { bind(sock_fd, s_addr, addr_len) } {
        println!("Bind socket failed!");
        return 0;
    }
    println!("Bind socket successed!");


    if -1 == unsafe { listen(sock_fd, SOMAXCONN) } {
        println!("Socket listen failed!");
        return 0;
    }
    println!("Listen socket successed!");
    
    
    sock_fd
}



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