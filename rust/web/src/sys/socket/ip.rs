use libc;
use std::{net, mem, fmt, hash, cmp};
use std::net::{hton, ntoh};
use super::consts;

#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[allow(deprecated)]
impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddr::V4(ref a) => a.fmt(fmt),
            IpAddr::V6(ref a) => a.fmt(fmt),
        }
    }
}

#[derive(Copy)]
pub struct Ipv4Addr(libc::in_addr);

#[derive(Copy)]
pub struct Ipv6Addr(libc::in6_addr);

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        let addr = libc::in_addr {
            s_addr: hton(((a as u32) << 24) |
                         ((b as u32) << 16) |
                         ((c as u32) <<  8) |
                          (d as u32)),
        });

        Ipv4Addr(addr)
    }

    pub fn from_std(std: &net::Ipv4Addr) -> Ipv4Addr {
        let ip = std.octets();
        Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    }

    pub fn any() -> Ipv4Addr {
        Ipv4Addr(libc::in_addr { s_addr: consts::INADDR_ANY })
    }

    pub fn to_std(&self) -> net::Ipv4Addr {
        let ip = self.octets();
        net::Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    }

    pub fn octets(&self) -> [u8; 4] {
        let bits = ntoh(self.s_addr);
        [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8]
    }

    /// Returns true for the special 'unspecified' address 0.0.0.0.
    pub fn is_unspecified(&self) -> bool {
        self.inner.s_addr == 0
    }

    /// Returns true if this is a loopback address (127.0.0.0/8).
    ///
    /// This property is defined by RFC 6890
    pub fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }

    /// Returns true if this is a private address.
    ///
    /// The private address ranges are defined in RFC1918 and include:
    ///
    ///  - 10.0.0.0/8
    ///  - 172.16.0.0/12
    ///  - 192.168.0.0/16
    pub fn is_private(&self) -> bool {
        match (self.octets()[0], self.octets()[1]) {
            (10, _) => true,
            (172, b) if b >= 16 && b <= 31 => true,
            (192, 168) => true,
            _ => false
        }
    }

    /// Returns true if the address is link-local (169.254.0.0/16).
    ///
    /// This property is defined by RFC 6890
    pub fn is_link_local(&self) -> bool {
        self.octets()[0] == 169 && self.octets()[1] == 254
    }

    /// Returns true if the address appears to be globally routable.
    /// See [iana-ipv4-special-registry][ipv4-sr].
    /// [ipv4-sr]: http://goo.gl/RaZ7lg
    ///
    /// The following return false:
    ///
    /// - private address (10.0.0.0/8, 172.16.0.0/12 and 192.168.0.0/16)
    /// - the loopback address (127.0.0.0/8)
    /// - the link-local address (169.254.0.0/16)
    /// - the broadcast address (255.255.255.255/32)
    /// - test addresses used for documentation (192.0.2.0/24, 198.51.100.0/24 and 203.0.113.0/24)
    /// - the unspecified address (0.0.0.0)
    pub fn is_global(&self) -> bool {
        !self.is_private() && !self.is_loopback() && !self.is_link_local() &&
        !self.is_broadcast() && !self.is_documentation() && !self.is_unspecified()
    }

    /// Returns true if this is a multicast address.
    ///
    /// Multicast addresses have a most significant octet between 224 and 239,
    /// and is defined by RFC 5771
    pub fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }

    /// Returns true if this is a broadcast address.
    ///
    /// A broadcast address has all octets set to 255 as defined in RFC 919.
    pub fn is_broadcast(&self) -> bool {
        self.octets()[0] == 255 && self.octets()[1] == 255 &&
        self.octets()[2] == 255 && self.octets()[3] == 255
    }

    /// Returns true if this address is in a range designated for documentation.
    ///
    /// This is defined in RFC 5737:
    ///
    /// - 192.0.2.0/24 (TEST-NET-1)
    /// - 198.51.100.0/24 (TEST-NET-2)
    /// - 203.0.113.0/24 (TEST-NET-3)
    pub fn is_documentation(&self) -> bool {
        match(self.octets()[0], self.octets()[1], self.octets()[2], self.octets()[3]) {
            (192, 0, 2, _) => true,
            (198, 51, 100, _) => true,
            (203, 0, 113, _) => true,
            _ => false
        }
    }

    /// Converts this address to an IPv4-compatible IPv6 address.
    ///
    /// a.b.c.d becomes ::a.b.c.d
    pub fn to_ipv6_compatible(&self) -> Ipv6Addr {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0,
                      ((self.octets()[0] as u16) << 8) | self.octets()[1] as u16,
                      ((self.octets()[2] as u16) << 8) | self.octets()[3] as u16)
    }

    /// Converts this address to an IPv4-mapped IPv6 address.
    ///
    /// a.b.c.d becomes ::ffff:a.b.c.d
    pub fn to_ipv6_mapped(&self) -> Ipv6Addr {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff,
                      ((self.octets()[0] as u16) << 8) | self.octets()[1] as u16,
                      ((self.octets()[2] as u16) << 8) | self.octets()[3] as u16)
    }
}

impl Clone for Ipv4Addr {
    fn clone(&self) -> Ipv4Addr { *self }
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

impl PartialEq for Ipv4Addr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.s_addr == other.s_addr
    }
}

impl Eq for Ipv4Addr {}

impl hash::Hash for Ipv4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.s_addr.hash(s)
    }
}

impl PartialOrd for Ipv4Addr {
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ipv4Addr {
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        self.octets().cmp(&other.octets())
    }
}

impl From<Ipv4Addr> for u32 {
    fn from(ip: Ipv4Addr) -> u32 {
        let ip = ip.octets();
        ((ip[0] as u32) << 24) + ((ip[1] as u32) << 16) + ((ip[2] as u32) << 8) + (ip[3] as u32)
    }
}

impl From<u32> for Ipv4Addr {
    fn from(ip: u32) -> Ipv4Addr {
        Ipv4Addr::new((ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8)
    }
}

impl From<[u8; 4]> for Ipv4Addr {
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}

impl Ipv6Addr {
    /// Creates a new IPv6 address from eight 16-bit segments.
    ///
    /// The result will represent the IP address a:b:c:d:e:f:g:h.
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16,
               h: u16) -> Ipv6Addr {
        let mut addr: libc::in6_addr = unsafe { mem::zeroed() };
        addr.s6_addr = [(a >> 8) as u8, a as u8,
                        (b >> 8) as u8, b as u8,
                        (c >> 8) as u8, c as u8,
                        (d >> 8) as u8, d as u8,
                        (e >> 8) as u8, e as u8,
                        (f >> 8) as u8, f as u8,
                        (g >> 8) as u8, g as u8,
                        (h >> 8) as u8, h as u8];
        Ipv6Addr { addr }
    }

    /// Returns the eight 16-bit segments that make up this address.
    pub fn segments(&self) -> [u16; 8] {
        let arr = &self.s6_addr;
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

    pub fn from_std(std: &net::Ipv6Addr) -> Ipv6Addr {
        let s = std.segments();
        Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    pub fn to_std(&self) -> net::Ipv6Addr {
        let s = self.segments();
        net::Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    /// Returns true for the special 'unspecified' address ::.
    ///
    /// This property is defined in RFC 6890.
    pub fn is_unspecified(&self) -> bool {
        self.segments() == [0, 0, 0, 0, 0, 0, 0, 0]
    }

    /// Returns true if this is a loopback address (::1).
    ///
    /// This property is defined in RFC 6890.
    pub fn is_loopback(&self) -> bool {
        self.segments() == [0, 0, 0, 0, 0, 0, 0, 1]
    }

    /// Returns true if the address appears to be globally routable.
    ///
    /// The following return false:
    ///
    /// - the loopback address
    /// - link-local, site-local, and unique local unicast addresses
    /// - interface-, link-, realm-, admin- and site-local multicast addresses
    pub fn is_global(&self) -> bool {
        match self.multicast_scope() {
            Some(Ipv6MulticastScope::Global) => true,
            None => self.is_unicast_global(),
            _ => false
        }
    }

    /// Returns true if this is a unique local address (IPv6).
    ///
    /// Unique local addresses are defined in RFC4193 and have the form fc00::/7.
    pub fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }

    /// Returns true if the address is unicast and link-local (fe80::/10).
    pub fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }

    /// Returns true if this is a deprecated unicast site-local address (IPv6
    /// fec0::/10).
    pub fn is_unicast_site_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfec0
    }

    /// Returns true if this is an address reserved for documentation
    /// This is defined to be 2001:db8::/32 in RFC RFC 3849
    pub fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }

    /// Returns true if the address is a globally routable unicast address.
    ///
    /// The following return false:
    ///
    /// - the loopback address
    /// - the link-local addresses
    /// - the (deprecated) site-local addresses
    /// - unique local addresses
    /// - the unspecified address
    /// - the address range reserved for documentation
    pub fn is_unicast_global(&self) -> bool {
        !self.is_multicast()
            && !self.is_loopback() && !self.is_unicast_link_local()
            && !self.is_unicast_site_local() && !self.is_unique_local()
            && !self.is_unspecified() && !self.is_documentation()
    }

    /// Returns the address's multicast scope if the address is multicast.
    pub fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None
            }
        } else {
            None
        }
    }

    /// Returns true if this is a multicast address.
    ///
    /// Multicast addresses have the form ff00::/8, and this property is defined
    /// by RFC 3956.
    #[stable(since = "1.7.0", feature = "ip_17")]
    pub fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }

    /// Converts this address to an IPv4 address. Returns None if this address is
    /// neither IPv4-compatible or IPv4-mapped.
    ///
    /// ::a.b.c.d and ::ffff:a.b.c.d become a.b.c.d
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn to_ipv4(&self) -> Option<Ipv4Addr> {
        match self.segments() {
            [0, 0, 0, 0, 0, f, g, h] if f == 0 || f == 0xffff => {
                Some(Ipv4Addr::new((g >> 8) as u8, g as u8,
                                   (h >> 8) as u8, h as u8))
            },
            _ => None
        }
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.segments() {
            // We need special cases for :: and ::1, otherwise they're formatted
            // as ::0.0.0.[01]
            [0, 0, 0, 0, 0, 0, 0, 0] => write!(fmt, "::"),
            [0, 0, 0, 0, 0, 0, 0, 1] => write!(fmt, "::1"),
            // Ipv4 Compatible address
            [0, 0, 0, 0, 0, 0, g, h] => {
                write!(fmt, "::{}.{}.{}.{}", (g >> 8) as u8, g as u8,
                       (h >> 8) as u8, h as u8)
            }
            // Ipv4-Mapped address
            [0, 0, 0, 0, 0, 0xffff, g, h] => {
                write!(fmt, "::ffff:{}.{}.{}.{}", (g >> 8) as u8, g as u8,
                       (h >> 8) as u8, h as u8)
            },
            _ => {
                fn find_zero_slice(segments: &[u16; 8]) -> (usize, usize) {
                    let mut longest_span_len = 0;
                    let mut longest_span_at = 0;
                    let mut cur_span_len = 0;
                    let mut cur_span_at = 0;

                    for i in 0..8 {
                        if segments[i] == 0 {
                            if cur_span_len == 0 {
                                cur_span_at = i;
                            }

                            cur_span_len += 1;

                            if cur_span_len > longest_span_len {
                                longest_span_len = cur_span_len;
                                longest_span_at = cur_span_at;
                            }
                        } else {
                            cur_span_len = 0;
                            cur_span_at = 0;
                        }
                    }

                    (longest_span_at, longest_span_len)
                }

                let (zeros_at, zeros_len) = find_zero_slice(&self.segments());

                if zeros_len > 1 {
                    fn fmt_subslice(segments: &[u16], fmt: &mut fmt::Formatter) -> fmt::Result {
                        if !segments.is_empty() {
                            try!(write!(fmt, "{:x}", segments[0]));
                            for &seg in &segments[1..] {
                                try!(write!(fmt, ":{:x}", seg));
                            }
                        }
                        Ok(())
                    }

                    try!(fmt_subslice(&self.segments()[..zeros_at], fmt));
                    try!(fmt.write_str("::"));
                    fmt_subslice(&self.segments()[zeros_at + zeros_len..], fmt)
                } else {
                    let &[a, b, c, d, e, f, g, h] = &self.segments();
                    write!(fmt, "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                           a, b, c, d, e, f, g, h)
                }
            }
        }
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
        self.s6_addr == other.s6_addr
    }
}

impl Eq for Ipv6Addr {}

impl hash::Hash for Ipv6Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.s6_addr.hash(s)
    }
}

impl PartialOrd for Ipv6Addr {
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ipv6Addr {
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}
