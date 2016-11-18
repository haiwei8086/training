
use super::{consts, ffi};
use super::super::{winapi};
use super::super::winapi::ws2def::SOCKADDR_IN;
use super::super::winapi::ws3

use std::{fmt, hash, mem, net, ptr};

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
