
use libc;
use std::{fmt, hash, mem, net};
use std::convert::From;
use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum NsIpAddr {
    V4(NsIpv4Addr),
    V6(NsIpv6Addr),
}

impl fmt::Display for NsIpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsIpAddr::V4(ref a) => a.fmt(fmt),
            NsIpAddr::V6(ref a) => a.fmt(fmt),
        }
    }
}


#[derive(Copy)]
pub struct NsIpv4Addr(pub libc::in_addr);


impl NsIpv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> NsIpv4Addr {
        let ip = libc::in_addr {
            s_addr: (((a as u32) << 24) |
                     ((b as u32) << 16) |
                     ((c as u32) <<  8) |
                      (d as u32)).to_be()
        };

        NsIpv4Addr(ip)
    }

    pub fn octets(&self) -> [u8; 4] {
        let bits = u32::from_be(self.0.s_addr);
        [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    pub fn from_std(std: &net::Ipv4Addr) -> NsIpv4Addr {
        let bits = std.octets();
        NsIpv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }

    pub fn to_std(&self) -> net::Ipv4Addr {
        let bits = self.octets();
        net::Ipv4Addr::new(bits[0], bits[1], bits[2], bits[3])
    }
}

impl PartialEq for NsIpv4Addr {
    fn eq(&self, other: &NsIpv4Addr) -> bool {
        self.0.s_addr == other.0.s_addr
    }
}

impl Eq for NsIpv4Addr {}

impl hash::Hash for NsIpv4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s_addr.hash(s)
    }
}

impl PartialOrd for NsIpv4Addr {
    fn partial_cmp(&self, other: &NsIpv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NsIpv4Addr {
    fn cmp(&self, other: &NsIpv4Addr) -> Ordering {
        self.octets().cmp(&other.octets())
    }
}

impl Clone for NsIpv4Addr {
    fn clone(&self) -> NsIpv4Addr { *self }
}

impl fmt::Display for NsIpv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(fmt, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}

impl fmt::Debug for NsIpv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl From<NsIpv4Addr> for u32 {
    fn from(ip: NsIpv4Addr) -> u32 {
        let ip = ip.octets();
        ((ip[0] as u32) << 24) + ((ip[1] as u32) << 16) + ((ip[2] as u32) << 8) + (ip[3] as u32)
    }
}

impl From<u32> for NsIpv4Addr {
    fn from(ip: u32) -> NsIpv4Addr {
        NsIpv4Addr::new((ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8)
    }
}


#[derive(Copy)]
pub struct NsIpv6Addr(pub libc::in6_addr);


impl NsIpv6Addr {
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> NsIpv6Addr {
        let mut addr: libc::in6_addr = unsafe { mem::zeroed() };
        addr.s6_addr = [(a >> 8) as u8, a as u8,
                        (b >> 8) as u8, b as u8,
                        (c >> 8) as u8, c as u8,
                        (d >> 8) as u8, d as u8,
                        (e >> 8) as u8, e as u8,
                        (f >> 8) as u8, f as u8,
                        (g >> 8) as u8, g as u8,
                        (h >> 8) as u8, h as u8];
        NsIpv6Addr(addr)
    }

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

    pub fn from_std(std: &net::Ipv6Addr) -> NsIpv6Addr {
        let s = std.segments();
        NsIpv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    pub fn to_std(&self) -> net::Ipv6Addr {
        let s = self.segments();
        net::Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }
}

impl PartialEq for NsIpv6Addr {
    fn eq(&self, other: &NsIpv6Addr) -> bool {
        self.0.s6_addr == other.0.s6_addr
    }
}

impl Eq for NsIpv6Addr {}

impl hash::Hash for NsIpv6Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.0.s6_addr.hash(s)
    }
}

impl PartialOrd for NsIpv6Addr {
    fn partial_cmp(&self, other: &NsIpv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NsIpv6Addr {
    fn cmp(&self, other: &NsIpv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}

impl fmt::Display for NsIpv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.to_std().fmt(fmt)
    }
}

impl fmt::Debug for NsIpv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl Clone for NsIpv6Addr {
    fn clone(&self) -> NsIpv6Addr { *self }
}

impl From<[u8; 16]> for NsIpv6Addr {
    fn from(octets: [u8; 16]) -> NsIpv6Addr {
        let mut inner: libc::in6_addr = unsafe { mem::zeroed() };
        inner.s6_addr = octets;
        NsIpv6Addr(inner)
    }
}
