#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_char, c_ushort, c_ulong};
use winapi::minwindef::LPVOID;
use winapi::minwindef::LPDWORD;
use winapi::minwinbase::OVERLAPPED;
use winapi::winnt::PVOID;
use winapi::guiddef::GUID;
use winapi::winnt::HANDLE;


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

// {0xb5367df1,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}
pub const WSAID_ACCEPTEX: GUID = GUID {
    Data1: 0xb5367df1,
    Data2: 0xcbac,
    Data3: 0x11cf,
    Data4: [0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92],
};

// {0xb5367df2,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}
pub const WSAID_GETACCEPTEXSOCKADDRS: GUID = GUID {
    Data1: 0xb5367df2,
    Data2: 0xcbac,
    Data3: 0x11cf,
    Data4: [0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92],
};

// {0xb5367df0,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}
pub const WSAID_TRANSMITFILE: GUID = GUID {
    Data1: 0xb5367df0,
    Data2: 0xcbac,
    Data3: 0x11cf,
    Data4: [0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92],
};

// {0xd9689da0,0x1f90,0x11d3,{0x99,0x71,0x00,0xc0,0x4f,0x68,0xc8,0x76}}
pub const WSAID_TRANSMITPACKETS: GUID = GUID {
    Data1: 0xd9689da0,
    Data2: 0x1f90,
    Data3: 0x11d3,
    Data4: [0x99,0x71,0x00,0xc0,0x4f,0x68,0xc8,0x76],
};

// {0x25a207b9,0xddf3,0x4660,{0x8e,0xe9,0x76,0xe5,0x8c,0x74,0x06,0x3e}}
pub const WSAID_CONNECTEX: GUID = GUID {
    Data1: 0x25a207b9,
    Data2: 0xddf3,
    Data3: 0x4660,
    Data4: [0x8e,0xe9,0x76,0xe5,0x8c,0x74,0x06,0x3e],
};

// {0x7fda2e11,0x8630,0x436f,{0xa0,0x31,0xf5,0x36,0xa6,0xee,0xc1,0x57}}
pub const WSAID_DISCONNECTEX: GUID = GUID {
    Data1: 0x7fda2e11,
    Data2: 0x8630,
    Data3: 0x436f,
    Data4: [0xa0,0x31,0xf5,0x36,0xa6,0xee,0xc1,0x57],
};

extern "system" {
    pub fn WSAIoctl(
        sockfd: u64,
        dwIoControlCode: u32,
        lpvInBuffer: LPVOID,
        cbInBuffer: u32,
        lpvOutBuffer: LPVOID,
        cbOutBuffer: u32,
        lpcbBytesReturned: LPDWORD,
        lpOverlapped: LPVOID,
        lpCompletionRoutine: LPVOID,
    ) -> i32;

    pub fn LPFN_AcceptEx(
        sListenSocket: u64,
        sAcceptSocket: u64,
        lpOutputBuffer: PVOID,
        dwReceiveDataLength: u32,
        dwLocalAddressLength: u32,
        dwRemoteAddressLength: u32,
        lpdwBytesReceived: *mut u32,
        lpOverlapped: *mut OVERLAPPED,
    ) -> bool;

    pub fn LPFN_GetAcceptExSockaddrs(
        lpOutputBuffer: PVOID,
        dwReceiveDataLength: u32,
        dwLocalAddressLength: u32,
        dwRemoteAddressLength: u32,
        LocalSockaddr: *mut sockaddr,
        LocalSockaddrLength: *mut i32,
        RemoteSockaddr: *mut sockaddr,
        RemoteSockaddrLength: *mut i32
    );

    pub fn LPFN_TransmitFile(
        hSocket: u64,
        hFile: HANDLE,
        nNumberOfBytesToWrite: u32,
        nNumberOfBytesPerSend: u32,
        lpOverlapped: *mut OVERLAPPED,
        lpTransmitBuffers: LPVOID,
        dwFlags: u32
    ) -> bool;

    pub fn LPFN_TransmitPackets(
        hSocket: u64,
        lpPacketArray: LPVOID,
        nElementCount: u32,
        nSendSize: u32,
        lpOverlapped: *mut OVERLAPPED,
        dwFlags: u32
    ) -> bool;

    pub fn LPFN_ConnectEx(
        sockfd: u64,
        name: sockaddr,
        namelen: i32,
        lpSendBuffer: PVOID,
        dwSendDataLength: u32,
        lpdwBytesSent: LPDWORD,
        lpOverlapped: *mut OVERLAPPED
    ) -> bool;

    pub fn LPFN_DisconnectEx(
        hSocket: u64,
        lpOverlapped: *mut OVERLAPPED,
        dwFlags: u32,
        reserved: u32
    ) -> bool;
}
