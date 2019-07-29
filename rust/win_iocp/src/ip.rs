#![allow(dead_code)]


use std::{fmt, net, hash};

use super::ffi::{in_addr, in6_addr};



#[derive(Clone, Copy)]
pub struct IPAddrV4(pub in_addr);

#[derive(Clone, Copy)]
pub struct IPAddrV6(pub in6_addr);


#[derive(Clone, Copy)]
pub enum IPAddr {
    V4(IPAddrV4),
    V6(IPAddrV6),
}


impl IPAddrV4 {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        let ip = (((a as u32) << 24) |
                  ((b as u32) << 16) |
                  ((c as u32) <<  8) |
                  ((d as u32) <<  0)).to_be();

        Self(in_addr { s_addr: ip })
    }

    pub fn octets(&self) -> [u8; 4] {
        self.0.s_addr.to_ne_bytes()
    }
}


impl IPAddrV6 {

    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self (
            in6_addr {
                s6_addr: [
                    (a >> 8) as u8, a as u8,
                    (b >> 8) as u8, b as u8,
                    (c >> 8) as u8, c as u8,
                    (d >> 8) as u8, d as u8,
                    (e >> 8) as u8, e as u8,
                    (f >> 8) as u8, f as u8,
                    (g >> 8) as u8, g as u8,
                    (h >> 8) as u8, h as u8
                ],
            }
        )
    }

    pub fn segments(&self) -> [u16; 8] {
        let arr = &self.0.s6_addr;
        [
            u16::from_be_bytes([arr[0], arr[1]]),
            u16::from_be_bytes([arr[2], arr[3]]),
            u16::from_be_bytes([arr[4], arr[5]]),
            u16::from_be_bytes([arr[6], arr[7]]),
            u16::from_be_bytes([arr[8], arr[9]]),
            u16::from_be_bytes([arr[10], arr[11]]),
            u16::from_be_bytes([arr[12], arr[13]]),
            u16::from_be_bytes([arr[14], arr[15]]),
        ]
    }

    pub fn to_ipv4(&self) -> Option<IPAddrV4> {
        match self.segments() {
            [0, 0, 0, 0, 0, f, g, h] if f == 0 || f == 0xffff => {
                Some(IPAddrV4::new((g >> 8) as u8, g as u8,
                                   (h >> 8) as u8, h as u8))
            },
            _ => None
        }
    }

    pub const fn octets(&self) -> [u8; 16] {
        self.0.s6_addr
    }
}



impl Eq for IPAddrV4 {}
impl Eq for IPAddrV6 {}


impl hash::Hash for IPAddrV4 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        {self.0.s_addr}.hash(s)
    }
}
impl hash::Hash for IPAddrV6 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) { self.0.s6_addr.hash(s) }
}


impl PartialEq for IPAddrV4 {
    fn eq(&self, other: &IPAddrV4) -> bool { self.0.s_addr == other.0.s_addr}
}
impl PartialEq for IPAddrV6 {
    fn eq(&self, other: &IPAddrV6) -> bool { self.0.s6_addr == other.0.s6_addr }
}


impl From<net::Ipv4Addr> for IPAddrV4 {
    fn from(addr: net::Ipv4Addr) -> Self {
        let octets = addr.octets();
        IPAddrV4::new(octets[0], octets[1], octets[2], octets[3])
    }
}
impl From<IPAddrV4> for net::Ipv4Addr {
    fn from(addr: IPAddrV4) -> Self {
        let octets = addr.octets();

        net::Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}
impl From<net::Ipv6Addr> for IPAddrV6 {
    fn from(addr: net::Ipv6Addr) -> Self {
        let [a, b, c, d, e, f, g, h] = addr.segments();
        IPAddrV6::new(a, b, c, d, e, f, g, h)
    }
}
impl From<IPAddrV6> for net::Ipv6Addr {
    fn from(addr: IPAddrV6) -> Self {
        net::Ipv6Addr::from(addr.segments())
    }
}
impl From<net::IpAddr> for IPAddr {
    fn from(addr: net::IpAddr) -> Self {
        match addr {
            net::IpAddr::V4(ip) => IPAddr::V4(IPAddrV4::from(ip)),
            net::IpAddr::V6(ip) => IPAddr::V6(IPAddrV6::from(ip)),
        }
    }
}
impl From<IPAddr> for net::IpAddr {
    fn from(addr: IPAddr) -> Self {
        match addr {
            IPAddr::V4(ip) => net::IpAddr::V4(net::Ipv4Addr::from(ip)),
            IPAddr::V6(ip) => net::IpAddr::V6(net::Ipv6Addr::from(ip)),
        }
    }
}



impl fmt::Display for IPAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IPAddr::V4(ip) => ip.fmt(fmt),
            IPAddr::V6(ip) => ip.fmt(fmt),
        }
    }
}
impl fmt::Display for IPAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = u32::from_be(self.0.s_addr);
        let octets = [(bits >> 24) as u8, (bits >> 16) as u8, (bits >> 8) as u8, bits as u8];
        write!(f, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
    }
}
impl fmt::Display for IPAddrV6 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                    fn fmt_subslice(segments: &[u16], fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                        if !segments.is_empty() {
                            write!(fmt, "{:x}", segments[0])?;
                            for &seg in &segments[1..] {
                                write!(fmt, ":{:x}", seg)?;
                            }
                        }
                        Ok(())
                    }

                    fmt_subslice(&self.segments()[..zeros_at], fmt)?;
                    fmt.write_str("::")?;
                    fmt_subslice(&self.segments()[zeros_at + zeros_len..], fmt)
                } else {
                    let &[a, b, c, d, e, f, g, h] = &self.segments();
                    write!(fmt, "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", a, b, c, d, e, f, g, h)
                }
            }
        }
    }
}
impl fmt::Debug for IPAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}