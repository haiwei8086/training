
use std::{mem, ptr};
use std::io::Error;
use std::os::unix::io::RawFd;
use libc;
use libc::{c_int};
use super::addr;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 15000;
const MAX_BUFFER: usize = 1000;
const MAX_LISTEN_QUEUE: i32 = 1000;
const MAX_EVENTS: i32 = 100;
const MAX_EPOLL: i32 = 1000;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}

extern {
    fn epoll_create(flags: c_int) -> c_int;
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

    let epfd = unsafe { epoll_create(MAX_EPOLL) };
    if epfd < 0 {
        println!("Create epoll field: {}", epfd);
        return;
    }

    add_event(epfd, listenfd, libc::EPOLLIN | libc::EPOLLOUT | libc::EPOLLET);

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

    for i in events {
        if i.events as i32 == libc::EPOLLERR || i.events as i32 == libc::EPOLLHUP {
            println!("Epoll error!");
            /*
            unsafe {
                libc::close(i.data as i32)
            };
            */
            continue;
        } else if listenfd as u64 == i.data && libc::EPOLLIN as u32 == i.events {
            handle_accpet(epfd, i.data as i32);
        } else if libc::EPOLLIN as u32 == i.events {
            handle_read(epfd, i.data as i32);
        } else if libc::EPOLLOUT as u32 == i.events {
            handle_write(epfd, i.data as i32, &("Welcome to web server".as_bytes()));
        }
    }
}

fn handle_accpet(epfd: RawFd, fd: RawFd) {

    let mut client_addr: libc::sockaddr = unsafe { mem::uninitialized() };
    let mut client_sock_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
    let clientfd = unsafe { libc::accept(fd, &mut client_addr, &mut client_sock_len) };
    if clientfd < 0 {
        match Error::last_os_error().raw_os_error() {
            Some(x) => if x != libc::EAGAIN && x != libc::EPROTO && x != libc::EINTR {
                println!("Accept connection failed: {:?}", clientfd);
                remove_event(epfd, fd, libc::EPOLLIN);
                return;
            },
            None => (),
        }
    }

    let addr: libc::sockaddr_in = unsafe { mem::transmute(client_addr) };
    println!("Accept a client connection");
    println!("Client family: {:?}", addr.sin_family);
    println!("Client ip: {:?}, port: {:?}", addr.sin_addr.s_addr, addr.sin_port);

    make_non_blocking(clientfd);
    add_event(epfd, clientfd, libc::EPOLLIN | libc::EPOLLET);
}

fn handle_read(epfd: RawFd, fd: RawFd) {

    let mut buf = unsafe {
        let mut array: [u8; MAX_BUFFER] = mem::uninitialized();
        array
    };

    /*
    let mut buf = Vec::<u8>::with_capacity(MAX_BUFFER);
    unsafe {
        buf.set_len(MAX_BUFFER)
    };
    */

    let mut len = 0;
    let mut read_count = 0;

    while true {
        len = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, MAX_BUFFER)};
        println!("Read....{:?}", len);
        read_count += len;
        if len <= 0 {
            break;
        }
    }

    if len == -1 {
        println!("Read error: {:?}", Error::last_os_error().raw_os_error());
        match Error::last_os_error().raw_os_error() {
            Some(x) => if x != libc::EAGAIN {
                if read_count < 0 {
                    println!("Read failed: {:?}", read_count);
                } else {
                    println!("Can not read any data, client closed.");
                }
                unsafe {
                    libc::close(fd)
                };
                // remove_event(epfd, fd, libc::EPOLLIN);
                return;
            },
            None => (),
        }
    }

    println!("Read count: {:?}, buf: {:?}", read_count, buf.len());
    println!("Read content: \n{:?}", String::from_utf8_lossy(&buf[0..read_count as usize]));

    modify_event(epfd, fd, libc::EPOLLOUT);
}

fn handle_write(epfd: RawFd, fd: RawFd, buf: &[u8]) {
    let write_len = unsafe {
        libc::write(fd, buf as *const _ as *const libc::c_void, buf.len())
    };

    if write_len < 0 {
        println!("Error number: {:?}", Error::last_os_error());
        println!("Write data failed: {:?}", write_len);
        unsafe {
            libc::close(fd)
        };
        remove_event(epfd, fd, libc::EPOLLOUT);
        return;
    }

    println!("Write data: {:?}", write_len);
    println!("Write end --------------------------------------");

    unsafe {
        libc::close(fd)
    };
    // remove_event(epfd, fd, libc::EPOLLOUT);
}

fn add_event(epfd: RawFd, fd: RawFd, state: libc::c_int){

    let mut event = EpollEvent {
        events: state as u32,
        data: fd as u64
    };

    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_ADD, fd, &mut event) } < 0 {
        println!("Add epoll event failed");
    }
}

fn modify_event(epfd: RawFd, fd: RawFd, state: libc::c_int){

    let mut event = EpollEvent {
        events: state as u32,
        data: fd as u64
    };

    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_MOD, fd, &mut event) } < 0 {
        println!("Modify epoll event failed");
    }
}

fn remove_event(epfd: RawFd, fd: RawFd, state: libc::c_int) {

    let mut event = EpollEvent {
        events: state as u32,
        data: fd as u64
    };

    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_DEL, fd, &mut event) } < 0 {
        println!("Remove epoll event failed");
    }
}
