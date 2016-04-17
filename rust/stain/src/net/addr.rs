
use libc;
use std::{net, mem, fmt, hash};
use super::ip::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Copy)]
pub enum InetAddr {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6),
}

impl InetAddr {
    pub fn from_std(std: &net::SocketAddr) -> InetAddr {
        let ip = match *std {
            net::SocketAddr::V4(ref addr) => IpAddr::V4(Ipv4Addr::from_std(&addr.ip())),
            net::SocketAddr::V6(ref addr) => IpAddr::V6(Ipv6Addr::from_std(&addr.ip())),
        };

        InetAddr::new(ip, std.port())
    }

    pub fn new(ip: IpAddr, port: u16) -> InetAddr {
        match ip {
            IpAddr::V4(ref ip) => {
                InetAddr::V4(libc::sockaddr_in {
                    sin_family: libc::AF_INET as libc::sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: ip.as_ptr(),
                    .. unsafe { mem::zeroed() }
                })
            }
            IpAddr::V6(ref ip) => {
                InetAddr::V6(libc::sockaddr_in6 {
                    sin6_family: libc::AF_INET6 as libc::sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_addr: ip.as_ptr(),
                    .. unsafe { mem::zeroed() }
                })
            }
        }
    }

    pub fn ip(&self) -> IpAddr {
        match *self {
            InetAddr::V4(ref a) => IpAddr::V4(Ipv4Addr(a.sin_addr)),
            InetAddr::V6(ref a) => IpAddr::V6(Ipv6Addr(a.sin6_addr)),
        }
    }

    pub fn port(&self) -> u16 {
        match *self {
            InetAddr::V4(ref a) => u16::from_be(a.sin_port),
            InetAddr::V6(ref a) => u16::from_be(a.sin6_port),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        match *self {
            InetAddr::V4(ref a) => net::SocketAddr::V4(
                net::SocketAddrV4::new(
                    Ipv4Addr(a.sin_addr).to_std(),
                    self.port())
            ),
            InetAddr::V6(ref a) => net::SocketAddr::V6(
                net::SocketAddrV6::new(
                    Ipv6Addr(a.sin6_addr).to_std(),
                    self.port(),
                    a.sin6_flowinfo,
                    a.sin6_scope_id)
            ),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl PartialEq for InetAddr {
    fn eq(&self, other: &InetAddr) -> bool {
        match (*self, *other) {
            (InetAddr::V4(ref a), InetAddr::V4(ref b)) => {
                a.sin_port == b.sin_port && a.sin_addr.s_addr == b.sin_addr.s_addr
            }
            (InetAddr::V6(ref a), InetAddr::V6(ref b)) => {
                a.sin6_port == b.sin6_port &&
                    a.sin6_addr.s6_addr == b.sin6_addr.s6_addr &&
                    a.sin6_flowinfo == b.sin6_flowinfo &&
                    a.sin6_scope_id == b.sin6_scope_id
            }
            _ => false,
        }
    }
}

impl Eq for InetAddr {
}

impl hash::Hash for InetAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            InetAddr::V4(ref a) => {
                (a.sin_family,
                    a.sin_port,
                    a.sin_addr.s_addr).hash(s)
            }
            InetAddr::V6(ref a) => {
                (a.sin6_family,
                    a.sin6_port,
                    &a.sin6_addr.s6_addr,
                    a.sin6_flowinfo,
                    a.sin6_scope_id).hash(s)
            }
        }
    }
}

impl Clone for InetAddr {
    fn clone(&self) -> InetAddr {
        *self
    }
}

impl fmt::Display for InetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InetAddr::V4(_) => write!(f, "{}:{}", self.ip(), self.port()),
            InetAddr::V6(_) => write!(f, "[{}]:{}", self.ip(), self.port()),
        }
    }
}
