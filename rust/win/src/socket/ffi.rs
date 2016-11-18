
use std::os::raw::{c_char, c_ushort, c_ulong};

type ADDRESS_FAMILY = c_ushort;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SOCKADDR {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: ADDRESS_FAMILY,
    pub sin_port: c_ushort,
    pub sin_addr: in_addr,
    pub sin_zero: [c_char; 8],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: ADDRESS_FAMILY,
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
