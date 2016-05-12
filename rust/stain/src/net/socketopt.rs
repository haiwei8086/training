use libc;

pub fn set_non_blocking(sockfd: RawFd) {
    let mut flags = unsafe { libc::fcntl(sockfd, libc::F_GETFL, 0) };
    if flags < 0 {
        println!("Can not get fd flag: {:?}", flags);
        return;
    }

    flags |= libc::O_NONBLOCK;
    if unsafe { libc::fcntl(sockfd, libc::F_SETFL, flags) } < 0 {
        println!("Set socket non-blocking failed");
    }
}

pub fn set_keep_alive(sockfd: RawFd) {

    let keepalive = unsafe {
        let yes = 1;
        libc::setsockopt(
            sockfd,
            libc::SOL_SOCKET,
            libc::SO_KEEPALIVE,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as libc::socklen_t)
    };
    if keepalive < 0 {
        println!("Set socket opt keepalive failed!");
        println!("{:?}", Error::last_os_error());
    }
}

pub fn set_reuse_addr(sockfd: RawFd) {

    let reuse = unsafe {
        let yes = 1;
        libc::setsockopt(
            sockfd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<c_int>() as libc::socklen_t)
    };
    if reuse < 0 {
        println!("Set socket opt re-use failed!");
        println!("{:?}", Error::last_os_error());
    }
}

pub fn set_linger(sockfd: RawFd) {
    let l = Linger {
        l_onoff: 1,
        l_linger: 5
    };

    let ptr: *const libc::c_void = unsafe { mem::transmute(&l) };
    let len = unsafe { mem::size_of::<Linger>() };

    let ret = unsafe {
        libc::setsockopt(sockfd, libc::SOL_SOCKET, libc::SO_LINGER, ptr, len as libc::socklen_t)
    };

    if ret < 0 {
        println!("Set socket opt failed!");
        println!("{:?}", Error::last_os_error());
    }
}
