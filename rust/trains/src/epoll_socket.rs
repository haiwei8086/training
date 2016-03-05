use std::os::unix::io::RawFd;

use super::libc;
use super::libc::{c_int};

use super::addr;
use super::epoll;
use super::epoll::{event_type, ctl_op, EpollEvent};


const MAX_QUEUE: c_int = 100;


pub fn run() {
    println!("Epoll Socket");

    let addr = addr::SockAddr::new(addr::IpAddr::new_v4(127, 0, 0, 1), 9000);
    let (ptr, len) = unsafe { addr.as_ffi_pair() };

    let listenfd = create_socket().unwrap();
    let res = unsafe { libc::bind(listenfd, ptr as *const libc::sockaddr, len) };
    let lres = unsafe { libc::listen(listenfd, MAX_QUEUE) };

    println!("socket bind res: {:?}", res);
    println!("socket listen res: {:?}", lres);
    println!("socket listenning on 9000");

    let epfd = epoll::create1(0).unwrap();

    let mut event = EpollEvent {
        data: listenfd as u64,
        events: (event_type::EPOLLIN | event_type::EPOLLET | event_type::EPOLLRDHUP)
    };
    match epoll::ctl(epfd, ctl_op::ADD, listenfd, &mut event) {
        Ok(()) => println!("Fd added sucessfully"),
        Err(e) => println!("Epoll CtlError during add: {}", e)
    };

    // Epoll wait
    let mut events = Vec::<EpollEvent>::with_capacity(100);
    unsafe { events.set_len(100); }
    match epoll::wait(epfd, &mut events[..], -1) {
        Ok(num_events) => {
            println!("{} epoll event(s) received", num_events);
            for x in 0..num_events {
                println!("epoll events.....");
            }
        }
        Err(e) => println!("Error on epoll::wait(): {}", e)
    }
}


fn create_socket() -> Result<RawFd, usize> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };

    if fd < 0 {
        return Err(fd as usize);
    }

    return Ok(fd);
}

/*
let socket_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
let ip = get_in_addr(127, 0, 0, 1);
let addr = get_socket_addr(ip, 9000);
let sockaddr: &libc::sockaddr = unsafe { mem::transmute(&addr) };


fn get_in_addr(a: u8, b: u8, c: u8, d: u8) -> libc::in_addr {
    let ip = (((a as u32) << 24) | ((b as u32) << 16) |
             ((c as u32) <<  8) | (d as u32)).to_be();
    libc::in_addr { s_addr: ip }
}

fn get_socket_addr(ip: libc::in_addr, port: u16) -> libc::sockaddr_in {
    libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: ip,
        .. unsafe { mem::zeroed() }
    }
}
*/
