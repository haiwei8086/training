
use std::{mem, ptr};
use std::io::Error;
use std::os::unix::io::RawFd;
use libc;
use libc::{c_int};
use super::addr;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 9000;
const MAX_BUFFER: usize = 1024 * 2;
const MAX_LISTEN_QUEUE: usize = 5;
const MAX_EVENTS: usize = 100;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Linger {
    pub l_onoff: c_int,
    pub l_linger: c_int
}

const EPOLLIN: u32 = 0x001;
const EPOLLPRI: u32 = 0x002;
const EPOLLOUT: u32 = 0x004;
const EPOLLRDNORM: u32 = 0x040;
const EPOLLWRBAND: u32 = 0x200;
const EPOLLRDBAND: u32 = 0x080;
const EPOLLWRNORM: u32 = 0x100;
const EPOLLMSG: u32 = 0x400;
const EPOLLERR: u32 = 0x008;
const EPOLLHUP: u32 = 0x010;
const EPOLLRDHUP: u32 = 0x2000;
const EPOLLWAKEUP: u32 = 1 << 29;
const EPOLLONESHOT: u32 = 1 << 30;
const EPOLLET: u32 = 1 << 31;
const EAGAIN: i32 = 11;
const EINTR: i32 = 4;

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
    println!("Create socket fd: {:?}", listenfd);

    set_socket_opt(listenfd);

    let addr = addr::SockAddr::new(addr::IpAddr::new_v4(IP[0],IP[1],IP[2],IP[3]), PORT);
    let (addr_ptr, socket_len) = unsafe { addr.as_ffi_pair() };

    if unsafe { libc::bind(listenfd, addr_ptr as *const libc::sockaddr, socket_len) } < 0 {
        println!("Socket bind failed");
        return Err(-2);
    }
    println!("Socket bind on port: {:?}", PORT);

    if unsafe { libc::listen(listenfd, MAX_LISTEN_QUEUE as i32) } < 0 {
        println!("Socket listenning failed!");
        return Err(-3);
    }
    println!("Socket listen on port: {:?}", PORT);

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

fn set_socket_opt(listenfd: RawFd) {

    let reuse = unsafe {
        let yes = 1;
        libc::setsockopt(
            listenfd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as libc::socklen_t)
    };
    if reuse < 0 {
        println!("Set socket opt re-use failed!");
        println!("{:?}", Error::last_os_error());
    }


    let keepalive = unsafe {
        let yes = 1;
        libc::setsockopt(
            listenfd,
            libc::SOL_SOCKET,
            libc::SO_KEEPALIVE,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as libc::socklen_t)
    };
    if keepalive < 0 {
        println!("Set socket opt keepalive failed!");
        println!("{:?}", Error::last_os_error());
    }

    let l = Linger {
        l_onoff: 1,
        l_linger: 5
    };

    let ptr: *const libc::c_void = unsafe {
        mem::transmute(&l)
    };
    let len = unsafe {
        mem::size_of::<Linger>()
    };

    let ret = unsafe {
        libc::setsockopt(listenfd, libc::SOL_SOCKET, libc::SO_LINGER, ptr, len as libc::socklen_t)
    };

    if ret < 0 {
        println!("Set socket opt failed!");
        println!("{:?}", Error::last_os_error());
    }
}

fn event_loop(listenfd: RawFd) {

    let epfd = unsafe { epoll_create1(0) };
    if epfd < 0 {
        println!("Create epoll field: {}", epfd);
        return;
    }
    println!("Create epoll successed: {:?}", epfd);

    let mut events: Vec<EpollEvent> = Vec::with_capacity(MAX_EVENTS);
    unsafe { events.set_len(MAX_EVENTS) };

    add_event(epfd, listenfd, EPOLLIN | EPOLLET | EPOLLRDHUP);

    loop {
        println!("Wait epoll EPOLLIN");
        let ret = unsafe { epoll_wait(epfd, events.as_mut_ptr(), MAX_EVENTS as i32, -1) };
        if ret < 0 {
            println!("Epoll wait failed: {}", ret);
            return;
        }
        println!("Have events count: {:?}", ret);

        for i in 0..ret {
            let item = events[i as usize];
            let clientfd = item.data as i32;

            println!("Event socket fd: {:?}  socket event: {:?}", item.data, item.events);

            if EPOLLERR == item.events || EPOLLHUP == item.events {
                println!("Epoll error: {}", item.events);
                continue;
            } else if listenfd as u64 == item.data{
                handle_accpet(epfd, clientfd);
            } else if EPOLLIN == item.events {
                handle_read(epfd, clientfd);
            } else if EPOLLOUT == item.events {
                handle_write(epfd, clientfd);
            } else {
                println!("Other epoll event: {:?}", item.events);
            }
        }

    }
}

fn handle_accpet(epfd: RawFd, fd: RawFd) {

    let mut client_addr: libc::sockaddr = unsafe { mem::uninitialized() };
    let mut client_sock_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    let clientfd = unsafe { libc::accept(fd, &mut client_addr, &mut client_sock_len) };
    if clientfd == -1 {
        println!("Accept connection failed: {:?}", clientfd);
        return;
    } else {
        let addr: libc::sockaddr_in = unsafe { mem::transmute(client_addr) };
        println!("Accept a client connection");
        println!("Client ip: {:?}, port: {:?}", addr.sin_addr.s_addr, addr.sin_port);

        //make_non_blocking(clientfd);
        add_event(epfd, clientfd, EPOLLIN | EPOLLET);
    }
}

fn handle_read(epfd: RawFd, fd: RawFd) {

    /*
    let mut buf = unsafe {
        let mut array: [u8; MAX_BUFFER] = mem::uninitialized();
        array
    };
    */

    let mut buf: Vec<u8> = Vec::with_capacity(MAX_BUFFER);
    unsafe {
        buf.set_len(MAX_BUFFER)
    };
    println!("Read socket fd: {:?}", fd);

    let read_count = unsafe {
        libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, MAX_BUFFER)
    };

    if read_count == -1 && read_count as i32 != EAGAIN {
        println!("Read error: {:?}", Error::last_os_error());

        /*
        let mut done = false;
        done = match Error::last_os_error().raw_os_error() {
            Some(EAGAIN) => {
                true
            },
            _ => {
                println!("Read failed: {:?}", read_count);
                unsafe {
                    libc::close(fd)
                };
                true
            },
        };

        if done {
            return;
        }
        */

        remove_event(epfd, fd);
        unsafe {
            libc::close(fd)
        };
        return;
    } else if read_count == 0 {
        println!("Can not read any data, may be client closed.");
        remove_event(epfd, fd);
        unsafe {
            libc::close(fd)
        };
        return;
    }

    println!("Read count: {:?}, buf: {:?}", read_count, buf.len());
    println!("Read content: \n{:?}", String::from_utf8_lossy(&buf[0..read_count as usize]));

    modify_event(epfd, fd, EPOLLOUT | EPOLLET);
}

fn handle_write(epfd: RawFd, fd: RawFd) {

    let buf: &[u8] = "Welcome to web server".as_bytes();

    println!("Write socket fd: {:?}", fd);
    let write_len = unsafe {
        libc::write(fd, buf as *const _ as *const libc::c_void, buf.len())
    };

    if write_len == -1 {
        if write_len as i32 == EINTR || write_len as i32 == EAGAIN {
            println!("Write blocked or eintr, need re-write.");
        } else {
            println!("Error number: {:?}", Error::last_os_error());
            println!("Write data failed: {:?}", write_len);
            remove_event(epfd, fd);
            unsafe {
                libc::close(fd)
            };
            return;
        }
    }

    println!("Write count: {:?}", write_len);
    println!("Write end --------------------------------------");

    remove_event(epfd, fd);
    unsafe {
        libc::close(fd)
    };
}

fn add_event(epfd: RawFd, fd: RawFd, state: u32){

    let mut event = EpollEvent {
        events: state,
        data: fd as u64
    };

    println!("Add epoll event epfd: {:?}  socket fd: {:?}  state: {:?}", epfd, fd, state);
    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_ADD, fd, &mut event) } < 0 {
        println!("Add epoll event failed");
    }
}

fn modify_event(epfd: RawFd, fd: RawFd, state: u32){

    let mut event = EpollEvent {
        events: state,
        data: fd as u64
    };

    println!("Modify epoll event epfd: {:?}  socket fd: {:?}  state: {:?}", epfd, fd, state);
    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_MOD, fd, &mut event) } < 0 {
        println!("Modify epoll event failed");
    }
}

fn remove_event(epfd: RawFd, fd: RawFd){

    let mut event = EpollEvent {
        events: 0,
        data: fd as u64,
    };

    println!("Remove epoll event epfd: {:?}  socket fd: {:?}  state: {:?}", epfd, fd, 0);
    if unsafe{ epoll_ctl(epfd, libc::EPOLL_CTL_MOD, fd, &mut event) } < 0 {
        println!("Remove epoll event failed");
    }
}
