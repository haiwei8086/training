
use std::{mem, ptr};
use std::os::unix::io::RawFd;
use libc;
use libc::{c_int};
use super::addr;

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 15000;
const MAX_LISTEN_QUEUE: i32 = 10;
const MAX_EVENTS: i32 = 100;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}

extern {
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
    fn epoll_wait(epfd: c_int, event: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
}

pub fn run() {
    println!("Web echo server");

    let listenfd = socket_bind().unwrap();

    event_loop(listenfd);
}

fn socket_bind() -> Result<RawFd, i32> {

    let listenfd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if listenfd < 0 {
        println!("Create socket failed: {}", listenfd);
        return Err(-1);
    }

    let addr = addr::SockAddr::new(addr::IpAddr::new_v4(IP[0],IP[1],IP[2],IP[3]), PORT);
    let (addr_ptr, socket_len) = unsafe { addr.as_ffi_pair() };

    if unsafe { libc::bind(listenfd, addr_ptr as *const libc::sockaddr, socket_len) } < 0 {
        println!("Socket bind failed");
        return Err(-2);
    }
    if unsafe { libc::listen(listenfd, MAX_LISTEN_QUEUE) } < 0 {
        println!("Socket listenning failed!");
        return Err(-3);
    }

    make_non_blocking(listenfd);

    return Ok(listenfd);
}

fn make_non_blocking(listenfd: RawFd) {
    let mut flags = unsafe { libc::fcntl(listenfd, libc::F_GETFL, 0) };
    if flags < 0 {
        println!("Can not get fd flag: {:?}", flags);
        return;
    }

    flags |= libc::O_NONBLOCK;
    if unsafe { libc::fcntl(listenfd, libc::F_SETFL, flags) } < 0 {
        println!("Set socket non-blocking failed");
    }
}

fn event_loop(listenfd: RawFd) {

    let epfd = unsafe { epoll_create1(0) };
    if epfd < 0 {
        println!("Create epoll field: {}", epfd);
        return;
    }

    add_event(epfd, listenfd, libc::EPOLLIN).unwrap();

    let mut events = Vec::<EpollEvent>::with_capacity(MAX_EVENTS as usize);
    unsafe { events.set_len(MAX_EVENTS as usize); }

    loop {
        let ret = unsafe { epoll_wait(epfd, events.as_mut_ptr(), MAX_EVENTS, -1) };
        if ret < 0 {
            println!("Epoll wait failed: {}", ret);
            return;
        }
        event_handle(epfd, listenfd, &events, ret);

    }
}

fn event_handle(epfd: RawFd, listenfd: RawFd, events: &[EpollEvent], num: i32) {
    println!("Have events: {:?}", num);
}

fn handle_accpet(epfd: &RawFd, listenfd: &RawFd) {}

fn handle_read(epfd: &RawFd, listenfd: &RawFd, buf: &Vec<u8>) {}

fn add_event(epfd: RawFd, fd: RawFd, state: libc::c_int) -> Result<i32, i32> {

    let mut event = EpollEvent {
        events: state as u32,
        data: fd as u64
    };

    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_ADD, fd, &mut event) } < 0 {
        println!("Add epoll event failed");
        return Err(-1);
    }

    return Ok(0);
}
