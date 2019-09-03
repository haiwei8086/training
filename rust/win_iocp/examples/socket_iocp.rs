#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


use std::{ptr, mem};
use std::os::raw::{c_int, c_char, c_ushort, c_ulong};
use std::net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr};
pub type sa_family_t = c_ushort;
pub type socklen_t = c_int;

use winapi::ctypes::c_void;
use winapi::um::winnt::HANDLE;
use winapi::shared::guiddef::GUID;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::ws2def::{AF_INET, SOCK_STREAM, SOCKADDR, SOCKADDR_IN, SIO_GET_EXTENSION_FUNCTION_POINTER};
use winapi::um::winsock2::{SOCKET, WSADATA, LPWSADATA, SOL_SOCKET, SO_REUSEADDR, SOCKET_ERROR, INVALID_SOCKET, SOMAXCONN, WSA_FLAG_OVERLAPPED, WSADESCRIPTION_LEN, WSASYS_STATUS_LEN};
use winapi::um::winnt::PVOID;
use winapi::shared::minwindef::{DWORD, BOOL, LPVOID, LPDWORD};
use winapi::um::minwinbase::{OVERLAPPED, LPOVERLAPPED, OVERLAPPED_ENTRY};
use winapi::um::mswsock::{SO_UPDATE_ACCEPT_CONTEXT};
use winapi::um::winbase::INFINITE;
use winapi::shared::ws2ipdef::SOCKADDR_IN6_LH;
use winapi::um::mswsock::{WSAID_ACCEPTEX, WSAID_GETACCEPTEXSOCKADDRS};


use winapi::um::winsock2::{WSAStartup, WSACleanup, WSASocketW, socket, setsockopt, bind, listen, closesocket, htons, inet_addr};
use winapi::shared::minwindef::{LOBYTE, HIBYTE};
use winapi::um::ioapiset::{CreateIoCompletionPort, GetQueuedCompletionStatus};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winsock2::WSAIoctl;
use winapi::um::winsock2::WSAGetLastError;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;


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
pub struct in_addr {
    pub s_addr: u32,
}


type LPFN_AcceptEx = unsafe extern "system" fn(SOCKET, SOCKET, PVOID, DWORD, DWORD, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;
type LPFN_GetAcceptExSockaddrs = unsafe extern "system" fn(PVOID, DWORD, DWORD, DWORD, *mut SOCKADDR, *mut c_int, *mut SOCKADDR, *mut c_int);


fn main() {
    let std_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9120);

    load_wsa();


    let iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
    let socket_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };

    println!("IOCP: {:?}, Socket: {:?}", iocp, socket_fd);


    let worker = unsafe { CreateThread(ptr::null_mut(), 0, Some(worker), iocp, 0, 0 as *mut _)};


    // Associate the listening socket with the completion port
    unsafe { CreateIoCompletionPort(socket_fd as HANDLE, iocp, 0, 0) };


    let (addr, addr_len) = socket_addr_to_ptrs(&std_addr);
    let bind_ret = unsafe { bind(socket_fd, addr, addr_len) };

    println!("bind: {:?}  WSAGetLastError: {:?}", bind_ret, unsafe { WSAGetLastError() });

    unsafe { 
        let mut reuse: c_char = 1;
        setsockopt(socket_fd, SOL_SOCKET, SO_REUSEADDR, &mut reuse as *const c_char, mem::size_of::<c_int>() as c_int)
    };
    unsafe { listen(socket_fd, SOMAXCONN) };

    println!("WSAGetLastError: {:?}", unsafe { WSAGetLastError() });


    let mut accept_ex_ret = 0 as usize;
    let mut bytes = 0;
    let io_ret = unsafe {
        WSAIoctl(
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &WSAID_ACCEPTEX as *const _ as *mut _,
            mem::size_of_val(&WSAID_ACCEPTEX) as DWORD,
            &mut accept_ex_ret as *mut _ as *mut c_void,
            mem::size_of_val(&accept_ex_ret) as DWORD,
            &mut bytes,
            0 as *mut _, 
            None)
    };
    if io_ret == SOCKET_ERROR {
        println!("WSAIoctl(LPFN_AcceptEx) failed.");
    }
    println!("WSAIoctl ret: {:?}", io_ret);

    let accept_ex = unsafe { mem::transmute::<_, LPFN_AcceptEx>(accept_ex_ret) };


    let mut get_accept_sock_addrs_ret = 0 as usize;
    let io_ret = unsafe {
        WSAIoctl(
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &WSAID_GETACCEPTEXSOCKADDRS as *const _ as *mut _,
            mem::size_of_val(&WSAID_GETACCEPTEXSOCKADDRS) as DWORD,
            &mut get_accept_sock_addrs_ret as *mut _ as *mut c_void,
            mem::size_of_val(&get_accept_sock_addrs_ret) as DWORD,
            &mut bytes,
            0 as *mut _, 
            None)
    };
    if io_ret == SOCKET_ERROR {
        println!("WSAIoctl(SIO_GET_EXTENSION_FUNCTION_POINTER) failed.");
    }
    println!("WSAIoctl ret: {:?}", io_ret);

    let get_accept_sock_addrs = unsafe { mem::transmute::<_, LPFN_GetAcceptExSockaddrs>(get_accept_sock_addrs_ret) };



    let accept_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };

    let mut over_lapped: OVERLAPPED = unsafe { mem::zeroed() };

    let mut out_buf: [i8; 1024] = unsafe { mem::zeroed() };
    let mut ex_bytes = 0;
    let sock_len = mem::size_of::<SOCKADDR_IN>() as u32;


    println!("out buf len: {:?}, sockaddr_in len: {:?}", out_buf.len(), sock_len);
    println!("Server socket: {:?}, Accept socket: {:?}", socket_fd, accept_fd);

    println!("WSAGetLastError: {:?}", unsafe { WSAGetLastError() });

    
    let ret = unsafe {
        accept_ex(
            socket_fd, 
            accept_fd, 
            &mut out_buf as *mut _ as PVOID,
            0,
            sock_len + 16,
            sock_len + 16,
            &mut ex_bytes,
            &mut over_lapped)
    };
    println!("AcceptEx ret: {:?}.", ret);
    println!("WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
    

    unsafe {
        WaitForSingleObject(worker, INFINITE)
    };


    unsafe { closesocket(socket_fd) };
    unsafe { closesocket(accept_fd) };
    unsafe { CloseHandle(iocp) };
    unsafe { WSACleanup() };
}


fn load_wsa() {
    let mut wsa_data = WSADATA {
        wVersion: 0,
        wHighVersion: 0,
        iMaxSockets: 0,
        iMaxUdpDg: 0,
        lpVendorInfo: &mut 0i8,
        szDescription: [0i8; WSADESCRIPTION_LEN + 1],
        szSystemStatus: [0i8; WSASYS_STATUS_LEN + 1],
    };

    unsafe { WSAStartup(0x0202, &mut wsa_data as LPWSADATA) };

    if LOBYTE(wsa_data.wVersion) != 2 && HIBYTE(wsa_data.wVersion) != 2 {
        unsafe { WSACleanup() };
        return;
    }

    println!("WinSock version: {:?}, {:?}", LOBYTE(wsa_data.wVersion), HIBYTE(wsa_data.wVersion));
}


fn socket_addr_to_ptrs(addr: &SocketAddr) -> (*const SOCKADDR, c_int) {
    match *addr {
        SocketAddr::V4(ref a) => {
            (a as *const _ as *const _, mem::size_of::<SOCKADDR_IN>() as c_int)
        }
        SocketAddr::V6(ref a) => {
            (a as *const _ as *const _, mem::size_of::<SOCKADDR_IN6_LH>() as c_int)
        }
    }
}


unsafe extern "system" fn worker(param: LPVOID) -> u32 {
    println!("Worker :{:?}", param);

    let iocp = param;
    let mut count = 0;
    let mut action_type = 10;
    let mut over_lapped = mem::zeroed();

    loop {
        GetQueuedCompletionStatus(
            iocp,
            &mut count,
            &mut action_type,
            &mut over_lapped,
            INFINITE
        );

        println!("Action type:{:?}", action_type);
        println!("Overlapped:{:?}", over_lapped);


        match action_type {
            0 => do_accept(),
            1 => do_recv(),
            2 => do_send(),
            _ => println!("Error: {:?}", WSAGetLastError()),
        }
    }
}


fn do_accept() {

}

fn do_recv() {

}

fn do_send() {}