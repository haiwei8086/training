
use super::{ffi, AddressFamily};
use std::{fmt, hash, mem, net};


#[derive(Copy)]
pub enum InetAddr {
    V4(ffi::sockaddr_in),
    V6(ffi::sockaddr_in6),
}

#[derive(Copy)]
pub enum SockAddr {
    Inet(InetAddr),
}

#[derive(Copy)]
pub struct IpV4Addr(ffi::in_addr);
#[derive(Copy)]
pub struct IpV6Addr(ffi::in6_addr);

pub enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

/*
    Impl IpV4Addr
*/
impl IpV4Addr {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> IpV4Addr {
        let ip = (((a as u32) << 24) |
                  ((b as u32) << 16) |
                  ((c as u32) << 8)  |
                  ((d as u32) << 0)).to_be();

        IpV4Addr(ffi::in_addr { s_addr: ip})
    }

    pub fn from_std(std: &net::Ipv4Addr) -> IpV4Addr {
        let bits = std.octets();

        IpV4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }

    pub fn any() -> IpV4Addr {
        IpV4Addr(ffi::in_addr { s_addr: 0 })
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

impl PartialEq for IpV4Addr {
    fn eq(&self, other: &IpV4Addr) -> bool {
        self.0.s_addr == other.0.s_addr
    }
}

impl Eq for IpV4Addr {}

impl hash::Hash for IpV4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s_addr.hash(s)
    }
}

impl Clone for IpV4Addr {
    fn clone(&self) -> IpV4Addr { *self }
}

impl fmt::Display for IpV4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl fmt::Debug for IpV4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/*
    Impl IpV6Addr
*/
impl IpV6Addr {

    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> IpV6Addr {
        let mut addr: ffi::in6_addr = unsafe { mem::zeroed() };
        addr.s6_addr = [(a >> 8) as u8, a as u8,
                        (b >> 8) as u8, b as u8,
                        (c >> 8) as u8, c as u8,
                        (d >> 8) as u8, d as u8,
                        (e >> 8) as u8, e as u8,
                        (f >> 8) as u8, f as u8,
                        (g >> 8) as u8, g as u8,
                        (h >> 8) as u8, h as u8];

        IpV6Addr(addr)
    }

    pub fn from_std(std: &net::Ipv6Addr) -> IpV6Addr {
        let s = std.segments();

        IpV6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
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

impl Clone for IpV6Addr {
    fn clone(&self) -> IpV6Addr { *self }
}

impl fmt::Display for IpV6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(fmt)
    }
}

impl fmt::Debug for IpV6Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/*
    Impl IpAddr
*/
impl IpAddr {
    pub fn new_v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(IpV4Addr::new(a, b, c, d))
    }

    pub fn new_v6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> IpAddr {
        IpAddr::V6(IpV6Addr::new(a, b, c, d, e, f, g, h))
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

impl fmt::Debug for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/*
    Impl InetAddr
*/
impl InetAddr {

    pub fn from_std(std: &net::SocketAddr) -> InetAddr {
        match *std {
            net::SocketAddr::V4(ref addr) => {
                InetAddr::V4(ffi::sockaddr_in {
                    sin_family: AddressFamily::Inet as ffi::sa_family_t,
                    sin_port: addr.port().to_be(),
                    sin_addr: IpV4Addr::from_std(addr.ip()).0,
                    .. unsafe { mem::zeroed() }
                })
            }
            net::SocketAddr::V6(ref addr) => {
                InetAddr::V6(ffi::sockaddr_in6 {
                    sin6_family: AddressFamily::Inet6 as ffi::sa_family_t,
                    sin6_port: addr.port().to_be(),
                    sin6_addr: IpV6Addr::from_std(addr.ip()).0,
                    sin6_flowinfo: addr.flowinfo(),
                    sin6_scope_id: addr.scope_id(),
                    .. unsafe { mem::zeroed() }
                })
            }
        }
    }

    pub fn new(ip: IpAddr, port: u16) -> InetAddr {
        match ip {
            IpAddr::V4(ref ip) => {
                InetAddr::V4(ffi::sockaddr_in {
                    sin_family: AddressFamily::Inet as ffi::sa_family_t,
                    sin_port: port.to_be(),
                    sin_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
            IpAddr::V6(ref ip) => {
                InetAddr::V6(ffi::sockaddr_in6 {
                    sin6_family: AddressFamily::Inet6 as ffi::sa_family_t,
                    sin6_port: port.to_be(),
                    sin6_addr: ip.0,
                    .. unsafe { mem::zeroed() }
                })
            }
        }
    }

    pub fn ip(&self) -> IpAddr {
        match *self {
            InetAddr::V4(ref addr) => IpAddr::V4(IpV4Addr(addr.sin_addr)),
            InetAddr::V6(ref addr) => IpAddr::V6(IpV6Addr(addr.sin6_addr)),
        }
    }

    pub fn port(&self) -> u16 {
        match *self {
            InetAddr::V4(ref addr) => u16::from_be(addr.sin_port),
            InetAddr::V6(ref addr) => u16::from_be(addr.sin6_port),
        }
    }

    pub fn to_std(&self) -> net::SocketAddr {
        match *self {
            InetAddr::V4(ref addr) => net::SocketAddr::V4(
                net::SocketAddrV4::new(
                    IpV4Addr(addr.sin_addr).to_std(),
                    self.port()
                )
            ),
            InetAddr::V6(ref addr) => net::SocketAddr::V6(
                net::SocketAddrV6::new(
                    IpV6Addr(addr.sin6_addr).to_std(),
                    self.port(),
                    addr.sin6_flowinfo,
                    addr.sin6_scope_id,
                )
            ),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }
}

impl Eq for InetAddr {}

impl Clone for InetAddr {
    fn clone(&self) -> InetAddr { *self }
}

impl PartialEq for InetAddr {
    fn eq(&self, other: &InetAddr) -> bool {
        match (*self, *other) {
            (InetAddr::V4(ref a), InetAddr::V4(ref b)) => {
                a.sin_addr.s_addr == b.sin_addr.s_addr
                && a.sin_port == b.sin_port
            }
            (InetAddr::V6(ref a), InetAddr::V6(ref b)) => {
                a.sin6_addr.s6_addr == b.sin6_addr.s6_addr
                && a.sin6_port == b.sin6_port
                && a.sin6_flowinfo == b.sin6_flowinfo
                && a.sin6_scope_id == b.sin6_scope_id
            }
            _ => false,
        }
    }
}

impl hash::Hash for InetAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            InetAddr::V4(ref a) => (
                a.sin_family,
                a.sin_port,
                a.sin_addr.s_addr).hash(s),
            InetAddr::V6(ref a) => (
                a.sin6_family,
                a.sin6_port,
                &a.sin6_addr.s6_addr,
                a.sin6_flowinfo,
                a.sin6_scope_id).hash(s),
        }
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

impl fmt::Debug for InetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/*
    Impl SockAddr
*/
impl SockAddr {

    pub fn new_inet(addr: InetAddr) -> SockAddr {
        SockAddr::Inet(addr)
    }

    pub fn family(&self) -> AddressFamily {
        match *self {
            SockAddr::Inet(InetAddr::V4(..)) => AddressFamily::Inet,
            SockAddr::Inet(InetAddr::V6(..)) => AddressFamily::Inet6,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self)
    }

    pub unsafe fn as_ptr_len(&self) -> (&ffi::sockaddr, ffi::socklen_t) {
        match *self {
            SockAddr::Inet(InetAddr::V4(ref a)) => (mem::transmute(a), mem::size_of::<ffi::sockaddr_in>() as ffi::socklen_t),
            SockAddr::Inet(InetAddr::V6(ref a)) => (mem::transmute(a), mem::size_of::<ffi::sockaddr_in6>() as ffi::socklen_t),
        }
    }
}

impl Eq for SockAddr {}

impl Clone for SockAddr {
    fn clone(&self) -> SockAddr { *self }
}

impl PartialEq for SockAddr {
    fn eq(&self, other: &SockAddr) -> bool {
        match (*self, *other) {
            (SockAddr::Inet(ref a), SockAddr::Inet(ref b)) => a == b,
        }
    }
}

impl hash::Hash for SockAddr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        match *self {
            SockAddr::Inet(ref a) => a.hash(s),
        }
    }
}

impl fmt::Display for SockAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SockAddr::Inet(ref inet) => inet.fmt(f),
        }
    }
}

impl fmt::Debug for SockAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
