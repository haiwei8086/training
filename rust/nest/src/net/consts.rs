
use libc::{c_int};


// Socket address family
pub const AF_UNIX: c_int  = 1;
pub const AF_LOCAL: c_int = 1;
pub const AF_INET: c_int  = 2;

#[cfg(target_os = "windows")]
pub const AF_INET6: ::c_int = 23;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub const AF_INET6: c_int = 10;
#[cfg(target_os = "netbsd")]
pub const AF_INET6: c_int = 24;
#[cfg(target_os = "openbsd")]
pub const AF_INET6: c_int = 26;
#[cfg(target_os = "freebsd")]
pub const AF_INET6: c_int = 28;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub const AF_INET6: c_int = 30;

pub const AF_NETLINK: c_int = 16;
pub const AF_PACKET: c_int = 17;


// Socket type
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_SEQPACKET: c_int = 5;
