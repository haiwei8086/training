use libc;


pub fn socket(domain: NsAddressFamily, ty: NsSocketTypes, protocol: i32) -> NsResult<RawFd> {
    let res = unsafe { libc::socket(domain as c_int, ty as c_int, protocol) };

    if res < 0 {
        println!("Create socket failed! {:?}", res);
        return Err(NsError::Unknow);
    }

    Ok(res)
}

pub fn socketpair(domain: NsAddressFamily, ty: NsSocketTypes, protocol: i32) -> NsResult<(RawFd, RawFd)> {
    let mut fds = [-1, -1];
    let res = unsafe { libc::socketpair(domain as c_int, ty as c_int, protocol, fds.as_mut_ptr()) };

    if res < 0 {
        println!("Create socket pair failed! {:?}", res);
        return Err(NsError::Unknow);
    }

    Ok((fds[0], fds[1]))
}


pub fn bind(sockfd: RawFd, addr: &NsSocketAddr) -> NsResult<RawFd> {
    let res = unsafe {
        let (ptr, len) = addr.as_ptr_len();
        libc::bind(sockfd, ptr, len)
    };

    if res < 0 {
        return Err(NsError::Unknow);
    }

    Ok(res)
}

pub fn listen(sockfd: RawFd, backlog: usize) -> NsResult<RawFd> {
    let res = unsafe { libc::listen(sockfd, backlog as c_int) };

    if res < 0 {
        return Err(NsError::Unknow);
    }

    Ok(res)
}
