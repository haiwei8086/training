#![allow(dead_code)]
#![allow(unused_imports)]


use std::{ mem, net, fmt, hash };
use winapi::shared::ws2def::{ AF_INET, AF_INET6 };
use super::ffi::{ sa_family_t, socklen_t, sockaddr_in, sockaddr_in6 };
use super::ip::{ IPAddr, IPAddrV4, IPAddrV6 };


#[derive(Copy, Clone)]
pub struct InetAddrV4(pub sockaddr_in);
#[derive(Copy, Clone)]
pub struct InetAddrV6(pub sockaddr_in6);


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum InetAddr {
    V4(InetAddrV4),
    V6(InetAddrV6),
}


// pub struct UnixAddr();

#[derive(Copy, Clone)]
pub enum ScoketAddr {
    Inet(InetAddr),
}



impl InetAddrV4 {

    pub fn new(ip: IPAddrV4, port: u16) -> Self {
        InetAddrV4(
            sockaddr_in {
                sin_family: AF_INET as sa_family_t,
                sin_port: port.to_be(),
                sin_addr: ip.0,
                .. unsafe { mem::zeroed() }
            }
        )
    }

    pub fn ip(&self) -> IPAddrV4 {
        IPAddrV4(self.0.sin_addr)
    }

    pub fn port(&self) -> u16 {
        u16::from_be(self.0.sin_port)
    }

}


impl InetAddrV6 {

    pub fn new(ip: IPAddrV6, port: u16) -> Self {
        InetAddrV6(
            sockaddr_in6 {
                sin6_family: AF_INET6 as sa_family_t,
                sin6_port: port.to_be(),
                sin6_addr: ip.0,
                .. unsafe { mem::zeroed() }
            }
        )
    }

    pub fn ip(&self) -> IPAddrV6 {
        IPAddrV6(self.0.sin6_addr)
    }

    pub fn port(&self) -> u16 {
        u16::from_be(self.0.sin6_port)
    }
}


impl InetAddr {

    pub fn new(ip: IPAddr, prot: u16) -> Self {
        match ip {
            IPAddr::V4(ip) => InetAddr::V4(InetAddrV4::new(ip, prot)),
            IPAddr::V6(ip) => InetAddr::V6(InetAddrV6::new(ip, prot)),
        }
    }

    
    pub fn ip(&self) -> IPAddr {
        match *self {
            InetAddr::V4(ref sa) => IPAddr::V4(sa.ip()),
            InetAddr::V6(ref sa) => IPAddr::V6(sa.ip()),
        }
    }


    pub fn port(&self) -> u16 {
        match *self {
            InetAddr::V4(ref sa) => sa.port(),
            InetAddr::V6(ref sa) => sa.port(),
        }
    }
}



impl From<net::SocketAddrV4> for InetAddrV4 {
    fn from(addr: net::SocketAddrV4) -> Self {
        InetAddrV4::new(IPAddrV4::from(*addr.ip()), addr.port())
    }
}
impl From<InetAddrV4> for net::SocketAddrV4 {
    fn from(addr: InetAddrV4) -> Self {
        net::SocketAddrV4::new(net::Ipv4Addr::from(addr.ip()), addr.port())
    }
}
impl From<net::SocketAddrV6> for InetAddrV6 {
    fn from(addr: net::SocketAddrV6) -> Self {
        InetAddrV6::new(IPAddrV6::from(*addr.ip()), addr.port())
    }
}
impl From<InetAddrV6> for net::SocketAddrV6 {
    fn from(addr: InetAddrV6) -> Self {
        net::SocketAddrV6::new(net::Ipv6Addr::from(addr.ip()), addr.port(), addr.0.sin6_flowinfo, addr.0.sin6_scope_id)
    }
}
impl From<net::SocketAddr> for InetAddr {
    fn from(addr: net::SocketAddr) -> Self {
        match addr {
            net::SocketAddr::V4(sa) => InetAddr::V4(InetAddrV4::from(sa)),
            net::SocketAddr::V6(sa) => InetAddr::V6(InetAddrV6::from(sa)),
        }
    }
}
impl From<InetAddr> for net::SocketAddr {
    fn from(addr: InetAddr) -> Self {
        match addr {
            InetAddr::V4(ref sa) => net::SocketAddr::V4(net::SocketAddrV4::from(*sa)),
            InetAddr::V6(ref sa) => net::SocketAddr::V6(net::SocketAddrV6::from(*sa)),
        }
    }
}


impl Eq for InetAddrV4 {}
impl Eq for InetAddrV6 {}


impl hash::Hash for InetAddrV4 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        (self.0.sin_port, self.0.sin_addr.s_addr).hash(s)
    }
}
impl hash::Hash for InetAddrV6 {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        (self.0.sin6_port, &self.0.sin6_addr.s6_addr,
         self.0.sin6_flowinfo, self.0.sin6_scope_id).hash(s)
    }
}


impl PartialEq for InetAddrV4 {
    fn eq(&self, other: &InetAddrV4) -> bool {
        self.0.sin_port == other.0.sin_port 
        && self.0.sin_addr.s_addr == other.0.sin_addr.s_addr
    }
}
impl PartialEq for InetAddrV6 {
    fn eq(&self, other: &InetAddrV6) -> bool {
        self.0.sin6_port == other.0.sin6_port 
        && self.0.sin6_addr.s6_addr == other.0.sin6_addr.s6_addr 
        && self.0.sin6_flowinfo == other.0.sin6_flowinfo 
        && self.0.sin6_scope_id == other.0.sin6_scope_id
    }
}


impl fmt::Display for InetAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}
impl fmt::Debug for InetAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}
impl fmt::Display for InetAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}
impl fmt::Debug for InetAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}
impl fmt::Display for InetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InetAddr::V4(ref a) => a.fmt(f),
            InetAddr::V6(ref a) => a.fmt(f),
        }
    }
}
impl fmt::Debug for InetAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InetAddr::V4(ref a) => a.fmt(f),
            InetAddr::V6(ref a) => a.fmt(f),
        }
    }
}