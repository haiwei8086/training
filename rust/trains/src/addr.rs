
use std::{fmt, hash, mem, net};
use super::libc;


pub const INADDR_ANY: u32 = 0;

#[derive(Copy)]
pub enum SockAddr {
    V4(libc::sockaddr_in),
    V6(libc::sockaddr_in6)
}

impl SockAddr {
    pub fn from_std(addr: &net::SocketAddr) -> SockAddr {
        let ip = match *addr {
            net::SocketAddr::V4(ref addr) => IpAddr::V4(Ipv4Addr::from_std(&addr.ip())),
            net::SocketAddr::V6(ref addr) => IpAddr::V6(Ipv6Addr::from_std(&addr.ip())),
        };

        SockAddr::new(ip, addr.port())
    }

    pub fn new(ip: IpAddr, port: u16) -> SockAddr {
        match ip {
            IpAddr::V4(ref ip) => {
                SockAddr::V4(libc::sockaddr_in {
                    sin_family: libc::AF_INET as libc::sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
            IpAddr::V6(ref ip) => {
                SockAddr::V6(libc::sockaddr_in6 {
                    sin6_family: libc::AF_INET6 as libc::sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
        }
    }
    /// Gets the IP address associated with this socket address.
    pub fn ip(&self) -> IpAddr {
        match *self {
            SockAddr::V4(ref sa) => IpAddr::V4(Ipv4Addr(sa.sin_addr)),
            SockAddr::V6(ref sa) => IpAddr::V6(Ipv6Addr(sa.sin6_addr)),
        }
    }

    /// Gets the port number associated with this socket address
    pub fn port(&self) -> u16 {
        match *self {
            SockAddr::V6(ref sa) => u16::from_be(sa.sin6_port),
            SockAddr::V4(ref sa) => u16::from_be(sa.sin_port),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        match *self {
            SockAddr::V4(ref sa) => net::SocketAddr::V4(
                net::SocketAddrV4::new(
                    Ipv4Addr(sa.sin_addr).to_std(),
                    self.port())),
            SockAddr::V6(ref sa) => net::SocketAddr::V6(
                net::SocketAddrV6::new(
                    Ipv6Addr(sa.sin6_addr).to_std(),
                    self.port(),
                    sa.sin6_flowinfo,
                    sa.sin6_scope_id)),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }

    pub unsafe fn as_ffi_pair(&self) -> (&libc::sockaddr, libc::socklen_t) {
        match *self {
            SockAddr::V4(ref addr) => (mem::transmute(addr), mem::size_of::<libc::sockaddr_in>() as libc::socklen_t),
            SockAddr::V6(ref addr) => (mem::transmute(addr), mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t),
        }
    }
}

impl PartialEq for SockAddr {
    fn eq(&self, other: &SockAddr) -> bool {
        match (*self, *other) {
            (SockAddr::V4(ref a), SockAddr::V4(ref b)) => {
                a.sin_port == b.sin_port &&
                    a.sin_addr.s_addr == b.sin_addr.s_addr
            }
            (SockAddr::V6(ref a), SockAddr::V6(ref b)) => {
                a.sin6_port == b.sin6_port &&
                    a.sin6_addr.s6_addr == b.sin6_addr.s6_addr &&
                    a.sin6_flowinfo == b.sin6_flowinfo &&
                    a.sin6_scope_id == b.sin6_scope_id
            }
            _ => false,
        }
    }
}

impl Eq for SockAddr {
}

impl hash::Hash for SockAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            SockAddr::V4(ref a) => {
                ( a.sin_family,
                  a.sin_port,
                  a.sin_addr.s_addr ).hash(s)
            }
            SockAddr::V6(ref a) => {
                ( a.sin6_family,
                  a.sin6_port,
                  &a.sin6_addr.s6_addr,
                  a.sin6_flowinfo,
                  a.sin6_scope_id ).hash(s)
            }
        }
    }
}

impl Clone for SockAddr {
    fn clone(&self) -> SockAddr {
        *self
    }
}

impl fmt::Display for SockAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SockAddr::V4(_) => write!(f, "{}:{}", self.ip(), self.port()),
            SockAddr::V6(_) => write!(f, "[{}]:{}", self.ip(), self.port()),
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    pub fn new_v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    pub fn new_v6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h))
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddr::V4(ref v4) => v4.fmt(f),
            IpAddr::V6(ref v6) => v6.fmt(f)
        }
    }
}

#[derive(Copy)]
pub struct Ipv4Addr(pub libc::in_addr);

impl Ipv4Addr {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        let ip = (((a as u32) << 24) |
                  ((b as u32) << 16) |
                  ((c as u32) <<  8) |
                  (d as u32)).to_be();
        Ipv4Addr(libc::in_addr { s_addr: ip })
    }

    pub fn from_std(std: &net::Ipv4Addr) -> Ipv4Addr {
        let bits = std.octets();
        Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }

    pub fn any() -> Ipv4Addr {
        Ipv4Addr(libc::in_addr { s_addr: INADDR_ANY })
    }

    pub fn octets(&self) -> [u8; 4] {
        let bits = u32::from_be(self.0.s_addr);
        [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    pub fn to_std(&self) -> net::Ipv4Addr {
        let bits = self.octets();
        net::Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv4Addr {
    fn clone(&self) -> Ipv4Addr { *self }
}

impl PartialEq for Ipv4Addr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.0.s_addr == other.0.s_addr
    }
}

impl Eq for Ipv4Addr {}

impl hash::Hash for Ipv4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s_addr.hash(s)
    }
}


#[derive(Copy)]
pub struct Ipv6Addr(pub libc::in6_addr);

impl Ipv6Addr {

    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        let mut addr: libc::in6_addr = unsafe { mem::zeroed() };
        addr.s6_addr = [(a >> 8) as u8, a as u8,
                        (b >> 8) as u8, b as u8,
                        (c >> 8) as u8, c as u8,
                        (d >> 8) as u8, d as u8,
                        (e >> 8) as u8, e as u8,
                        (f >> 8) as u8, f as u8,
                        (g >> 8) as u8, g as u8,
                        (h >> 8) as u8, h as u8];
        Ipv6Addr(addr)
    }

    pub fn from_std(std: &net::Ipv6Addr) -> Ipv6Addr {
        let s = std.segments();
        Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    /// Return the eight 16-bit segments that make up this address
    pub fn segments(&self) -> [u16; 8] {
        let arr = &self.0.s6_addr;
        [
            (arr[0] as u16) << 8 | (arr[1] as u16),
            (arr[2] as u16) << 8 | (arr[3] as u16),
            (arr[4] as u16) << 8 | (arr[5] as u16),
            (arr[6] as u16) << 8 | (arr[7] as u16),
            (arr[8] as u16) << 8 | (arr[9] as u16),
            (arr[10] as u16) << 8 | (arr[11] as u16),
            (arr[12] as u16) << 8 | (arr[13] as u16),
            (arr[14] as u16) << 8 | (arr[15] as u16),
        ]
    }

    pub fn to_std(&self) -> net::Ipv6Addr {
        let s = self.segments();
        net::Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(fmt)
    }
}

impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for Ipv6Addr {
    fn clone(&self) -> Ipv6Addr { *self }
}

impl PartialEq for Ipv6Addr {
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.0.s6_addr == other.0.s6_addr
    }
}

impl Eq for Ipv6Addr {}

impl hash::Hash for Ipv6Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s6_addr.hash(s)
    }
}
