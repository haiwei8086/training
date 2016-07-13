use std::{mem, io};
use std::os::unix::io::RawFd;

use libc;

use NsResult;
use NsError;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Linger {
    pub l_onoff: i32,
    pub l_linger: i32
}

pub fn get_flags(sockfd: RawFd) -> NsResult<libc::c_int> {
    let flags = unsafe { libc::fcntl(sockfd, libc::F_GETFL, 0) };
    if flags < 0 {
        println!("Can not get fd flag: {:?}", flags);
        return Err(NsError::Unknow);
    }

    Ok(flags)
}

pub fn set_nonblocking(sockfd: RawFd) -> NsResult<i32> {
    let mut flags = get_flags(sockfd).unwrap();

    flags |= libc::O_NONBLOCK;
    if unsafe { libc::fcntl(sockfd, libc::F_SETFL, flags) } < 0 {
        println!("Set socket non-blocking failed");
        return Err(NsError::Unknow);
    }

    return Ok(0);
}

pub fn set_reuse(sockfd: RawFd) -> NsResult<i32> {

    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            sockfd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<i32>() as libc::socklen_t)
    };

    if ret < 0 {
        println!("Set socket opt re-use failed!");
        println!("{:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn set_keepalive(sockfd: RawFd) -> NsResult<i32> {
    let ret = unsafe {
        let yes = 1;
        libc::setsockopt(
            sockfd,
            libc::SOL_SOCKET,
            libc::SO_KEEPALIVE,
            &yes as *const _ as *const libc::c_void,
            mem::size_of::<i32>() as libc::socklen_t)
    };
    if ret < 0 {
        println!("Set socket opt keepalive failed!");
        println!("{:?}", io::Error::last_os_error());

        return Err(NsError::Unknow);
    }

    Ok(ret)
}

pub fn set_linger(sockfd: RawFd, onoff: i32, linger: i32) -> NsResult<i32> {
    let l = Linger {
        l_onoff: onoff,
        l_linger: linger
    };

    let ptr: *const libc::c_void = unsafe { mem::transmute(&l) };
    let len = mem::size_of::<Linger>();

    let ret = unsafe {
        libc::setsockopt(
            sockfd,
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            ptr,
            len as libc::socklen_t)
    };

    if ret < 0 {
        println!("Set socket opt failed!");
        println!("{:?}", io::Error::last_os_error());
        return Err(NsError::Unknow);
    }

    Ok(ret)
}
