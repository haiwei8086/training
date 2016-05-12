
use libc;
use std::os::unix::io::RawFd;
use std::io::Error;

pub mod ip;
pub mod addr;

/*
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SocketDomain {
    Unix = libc::AF_UNIX,
    Inet = libc::AF_INET,
    Inet6 = libc::AF_INET6,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Netlink = consts::AF_NETLINK,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Packet = consts::AF_PACKET,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SocketType {
    Stream = libc::SOCK_STREAM,
    Datagrams = libc::SOCK_DGRAM,
    //SeqPacket = libc::SOCK_SEQPACKET,
    Raw = libc::SOCK_RAW,
    //Rdm = libc::SOCK_RDM,
}


pub fn socket(domain: SocketDomain, socketType: SocketType, protocol: i32) -> Result<RawFd, usize> {
    let fd = unsafe { libc::socket(domain,  socketType, protocol) };

    if fd < 0 {
        return Err(fd);
    }

    return Ok(fd);
}
*/
