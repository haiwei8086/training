
use std::os::unix::io::RawFd;
use std::{mem, ptr};
use libc;

const MAX_EVENT_COUNT: usize = 64;

pub fn run() {
    let result = bind_socket(&[127, 0, 0, 1], 15000).unwrap();
    println!("Server runing on port 15000...");

    let queues = create_queue().unwrap();
    kevent_register(queues, result).unwrap();

    wait_event(queues);
}

pub fn bind_socket(ip: &[u8], port: u16) -> Result<RawFd, i32> {

    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    println!("Create socket: {}", fd);

    if fd < 0 {
        return Err(fd);
    }

    let fd_len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
    let addr = &get_addr(ip, port);

    let result = unsafe { libc::bind(fd, addr as *const libc::sockaddr, fd_len) };
    println!("Bind socket: {}", result);

    if result < 0 {
        return Err(result);
    }

    println!("Set non blocking: {}", make_socket_non_blocking(fd).unwrap());

    return Ok(result);
}


fn get_addr(ip: &[u8], port: u16) -> libc::sockaddr {

    let ip_buf = (((ip[0] as u32) << 24) |
              ((ip[1] as u32) << 16) |
              ((ip[2] as u32) <<  8) |
              (ip[3] as u32)).to_be();

    let in_addr = libc::in_addr { s_addr: ip_buf};

    let addr = libc::sockaddr_in {
        sin_family: libc::AF_INET as libc::sa_family_t,
        sin_port: port.to_be(),
        sin_addr: in_addr,
        .. unsafe { mem::zeroed() }
    };

    let sock_addr: libc::sockaddr = unsafe { mem::transmute(addr) };

    return sock_addr;
}


fn make_socket_non_blocking(fd: RawFd) -> Result<i32, i32> {

    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL, 0) };
    if flags < 0 {
        return Err(flags);
    }

    let result = unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
    if result < 0 {
        return Err(result);
    }

    return Ok(result);
}


fn create_queue() -> Result<i32, i32> {

    let queue = unsafe { libc::kqueue() };
    println!("Create queue: {}", queue);

    if queue < 0 {
        return Err(queue);
    }

    return Ok(queue);
}

fn kevent_register(queuefd: RawFd, fd: RawFd) -> Result<i32, i32> {

    let mut events: Vec<libc::kevent> = Vec::new();

    events.push(libc::kevent {
        ident: fd as libc::uintptr_t,
        filter: libc::EVFILT_READ,
        flags: libc::EV_ADD,
        fflags: 0,
        data: 0,
        udata: ptr::null_mut() as *mut libc::c_void
    });

    let result = unsafe {
        libc::kevent(queuefd, events.as_ptr(), 1, events.as_mut_ptr(), 0, ptr::null())
    };
    println!("Register kevent: {}", result);

    if result < 0 {
        return Err(result);
    }

    return Ok(result);
}

fn wait_event (queuefd: RawFd) {
    let mut events: Vec<libc::kevent> = Vec::with_capacity(MAX_EVENT_COUNT);

    loop {
        println!("Waiting connection...");

        let ret = unsafe {
            libc::kevent(
                queuefd,
                events.as_ptr(),
                0,
                events.as_mut_ptr(),
                MAX_EVENT_COUNT as libc::c_int,
                ptr::null()
            )
        };
        if ret < 0 {
            println!("Wait event faild: {}", ret);
            return;
        }

        handle_events(queuefd, &events, ret);
    }
}

fn handle_events(queue: RawFd, event: &Vec<libc::kevent>, nevents: i32) {
    for i in 0..nevents {
        println!("Handler event: {}", i);
    }
}
