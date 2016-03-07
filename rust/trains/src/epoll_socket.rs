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
