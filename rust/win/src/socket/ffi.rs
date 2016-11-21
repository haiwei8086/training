#![warn(non_camel_case_types)]

use std::os::raw::{c_int, c_char, c_ushort, c_ulong};

pub type sa_family_t = c_ushort;
pub type socklen_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: c_ushort,
    pub sin_addr: in_addr,
    pub sin_zero: [c_char; 8],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: c_ushort,
    pub sin6_flowinfo: c_ulong,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}
