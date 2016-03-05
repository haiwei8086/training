use libc;
use libc::{c_int};
use nix::sys::socket::{SockAddr, InetAddr};

use std::{net, mem};
use std::str::FromStr;
use std::os::unix::io::RawFd;

extern "C" {
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
    fn epoll_wait(epfd: c_int, event: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
}

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}


pub fn run(){

    let sockfd: RawFd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    println!("Create sofcket fd: {:?}", sockfd);


    let set_result = unsafe { libc::fcntl(sockfd, libc::F_SETFL, libc::O_NONBLOCK) };
    println!("Set socket non-block: {:?}", set_result);

    let actual: net::SocketAddr = FromStr::from_str("0.0.0.0:3000").unwrap();
    let (ptr, len) = std_to_ffi_addr(&actual);

    /*
    let addr = SockAddr::Inet(InetAddr::from_std(&actual));
    let ptr: &libc::sockaddr = unsafe{ mem::transmute(&addr) };
    let len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
    */


    return;

    let bind_result = unsafe { libc::bind(sockfd, ptr, len) };
    println!("bind socket result: {:?}", bind_result);

    let lister_result = unsafe { libc::listen(sockfd, 10) };
    println!("listen result: {:?}", lister_result);

    // epoll --------------------------------------
    let epfd: RawFd = unsafe { epoll_create1(0) };
    println!("create epoll: {:?}", epfd);

    let mut event = EpollEvent {
        data: sockfd as u64,
        events: libc::EPOLLIN as u32
    };

    println!("create epoll event");

    let ep_ctl = unsafe { epoll_ctl(epfd, libc::EPOLL_CTL_ADD, sockfd, &mut event) };

    let mut events = Vec::<EpollEvent>::with_capacity(100);
    unsafe { events.set_len(100); }

    for x in 0..100 {
        let wait_res = unsafe { epoll_wait(epfd, events.as_mut_ptr(), events.len() as c_int, 2000) };
        println!("epoll wait result: {:?}", wait_res);
    }
}


fn std_to_ffi_addr(actual: &net::SocketAddr) -> (&libc::sockaddr, libc::socklen_t) {
    println!("socket std address: {:?}", actual);

    let addr = SockAddr::Inet(InetAddr::from_std(&actual));
    let ptr: &libc::sockaddr = unsafe{ mem::transmute(&addr) };
    let len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    println!("socket address: {:?}", ptr.sa_family);

    (ptr, len)
}
