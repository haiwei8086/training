use std::{fmt, hash, mem, net};

use libc;
use libc::{sa_family_t};

use super::consts;
use super::ip::{ NsIpAddr, NsIpv4Addr, NsIpv6Addr };


#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum NsAddressFamily {
    Unix = consts::os::AF_UNIX,
    Inet = consts::os::AF_INET,
    Inet6 = consts::os::AF_INET6,
}

#[derive(Clone)]
pub enum NsSocketAddr {
    Inet(NsInetAddr),
    Unix(NsUnixAddr),
}

#[derive(Copy)]
pub enum NsInetAddr {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6),
}

#[derive(Copy)]
pub struct NsUnixAddr(pub libc::sockaddr_un, pub usize);

impl NsInetAddr {
    pub fn new(ip: NsIpAddr, port: u16) -> NsInetAddr {
        match ip {
            NsIpAddr::V4(ref ip) => {
                NsInetAddr::V4(libc::sockaddr_in {
                    sin_family: NsAddressFamily::Inet as sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: ip.0,
                    ..unsafe { mem::zeroed() }
                })
            },
            NsIpAddr::V6(ref ip) => {
                NsInetAddr::V6(libc::sockaddr_in6 {
                    sin6_family: NsAddressFamily::Inet6 as sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_addr: ip.0,
                    ..unsafe { mem::zeroed()}
                })
            },
        }
    }

    pub fn from_std(std: &net::SocketAddr) -> NsInetAddr {
        match *std {
            net::SocketAddr::V4(ref addr) => {
                NsInetAddr::V4(libc::sockaddr_in {
                    sin_family: NsAddressFamily::Inet as sa_family_t,
                    sin_port: addr.port().to_be(),
                    sin_addr: NsIpv4Addr::from_std(addr.ip()).0,
                    ..unsafe { mem::zeroed() }
                })
            },
            net::SocketAddr::V6(ref addr) => {
                NsInetAddr::V6(libc::sockaddr_in6 {
                    sin6_family: NsAddressFamily::Inet6 as sa_family_t,
                    sin6_port: addr.port().to_be(),
                    sin6_addr: NsIpv6Addr::from_std(addr.ip()).0,
                    sin6_flowinfo: addr.flowinfo(),
                    sin6_scope_id: addr.scope_id(),
                    ..unsafe { mem::zeroed() }
                })
            },
        }
    }

    pub fn ip(&self) -> NsIpAddr {
        match *self {
            NsInetAddr::V4(ref addr) => NsIpAddr::V4(NsIpv4Addr(addr.sin_addr)),
            NsInetAddr::V6(ref addr) => NsIpAddr::V6(NsIpv6Addr(addr.sin6_addr)),
        }
    }

    pub fn port(&self) -> u16 {
        match *self {
            NsInetAddr::V4(ref addr) => u16::from_be(addr.sin_port),
            NsInetAddr::V6(ref addr) => u16::from_be(addr.sin6_port),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        match *self {
            NsInetAddr::V4(ref addr) => net::SocketAddr::V4(
                net::SocketAddrV4::new(
                    NsIpv4Addr(addr.sin_addr).to_std(),
                    self.port()
                )
            ),
            NsInetAddr::V6(ref addr) => net::SocketAddr::V6(
                net::SocketAddrV6::new(
                    NsIpv6Addr(addr.sin6_addr).to_std(),
                    self.port(),
                    addr.sin6_flowinfo,
                    addr.sin6_scope_id
                )
            ),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl PartialEq for NsInetAddr {
    fn eq(&self, other: &NsInetAddr) -> bool {
        match (*self, *other) {
            (NsInetAddr::V4(ref a), NsInetAddr::V4(ref b)) => {
                a.sin_port == b.sin_port && a.sin_addr.s_addr == b.sin_addr.s_addr
            },
            (NsInetAddr::V6(ref a), NsInetAddr::V6(ref b)) => {
                a.sin6_port == b.sin6_port
                && a.sin6_addr.s6_addr == b.sin6_addr.s6_addr
                && a.sin6_flowinfo == b.sin6_flowinfo
                && a.sin6_scope_id == b.sin6_scope_id
            }
            _ => false,
        }
    }
}

impl Eq for NsInetAddr {}

impl hash::Hash for NsInetAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            NsInetAddr::V4(ref a) => {
                (a.sin_family, a.sin_port, a.sin_addr.s_addr).hash(s)
            }
            NsInetAddr::V6(ref a) => {
                ( a.sin6_family,
                  a.sin6_port,
                  &a.sin6_addr.s6_addr,
                  a.sin6_flowinfo,
                  a.sin6_scope_id ).hash(s)
            }
        }
    }
}

impl Clone for NsInetAddr {
    fn clone(&self) -> NsInetAddr { *self }
}

impl fmt::Display for NsInetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsInetAddr::V4(_) => write!(f, "{}:{}", self.ip(), self.port()),
            NsInetAddr::V6(_) => write!(f, "[{}]:{}", self.ip(), self.port()),
        }
    }
}

impl fmt::Debug for NsInetAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl NsUnixAddr {
    pub fn new() -> NsUnixAddr {
        NsUnixAddr(libc::sockaddr_un {
            sun_family: NsAddressFamily::Unix as sa_family_t,
            ..unsafe { mem::zeroed() }
        }, 0)
    }

    pub fn len() -> usize {
        0
    }
}


impl Clone for NsUnixAddr {
    fn clone(&self) -> NsUnixAddr { *self }
}

impl NsSocketAddr {
    pub fn new_inet(addr: NsInetAddr) -> NsSocketAddr {
        NsSocketAddr::Inet(addr)
    }

    pub fn new_unix() -> NsSocketAddr {
        NsSocketAddr::Unix(NsUnixAddr::new())
    }

    pub fn family(&self) -> NsAddressFamily {
        match *self {
            NsSocketAddr::Inet(NsInetAddr::V4(..)) => NsAddressFamily::Inet,
            NsSocketAddr::Inet(NsInetAddr::V6(..)) => NsAddressFamily::Inet6,
            NsSocketAddr::Unix(..) => NsAddressFamily::Unix,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }

    pub unsafe fn as_ptr_len(&self) -> (&libc::sockaddr, libc::socklen_t) {
        match *self {
            NsSocketAddr::Inet(NsInetAddr::V4(ref addr)) => {
                (mem::transmute(addr), mem::size_of::<libc::sockaddr_in>() as libc::socklen_t)
            },
            NsSocketAddr::Inet(NsInetAddr::V6(ref addr)) => {
                (mem::transmute(addr), mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t)
            },
            NsSocketAddr::Unix(NsUnixAddr(ref addr, len)) => {
                (mem::transmute(addr), (len + mem::size_of::<libc::sa_family_t>()) as libc::socklen_t)
            }
        }
    }
}

impl fmt::Display for NsSocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsSocketAddr::Inet(ref a) => write!(f, "{}:{}", a.ip(), a.port()),
            _ => write!(f, "Unknow addr"),
        }
    }
}
impl fmt::Debug for NsSocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
