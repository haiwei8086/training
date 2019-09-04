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


pub const BUFF_SIZE: u32 = 1024;

type LPFN_AcceptEx = unsafe extern "system" fn(SOCKET, SOCKET, PVOID, DWORD, DWORD, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;
type LPFN_GetAcceptExSockaddrs = unsafe extern "system" fn(PVOID, DWORD, DWORD, DWORD, *mut SOCKADDR, *mut c_int, *mut SOCKADDR, *mut c_int);


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


pub struct MainContext 
{
    pub socket_fd: SOCKET,
    pub iocp: HANDLE,
    
    pub accept_ex_fn: LPFN_AcceptEx,
    pub get_accept_sock_addrs_fn: LPFN_GetAcceptExSockaddrs,
}

impl MainContext {
    pub fn new() -> Self {
        MainContext {
            socket_fd: INVALID_SOCKET,
            iocp: INVALID_HANDLE_VALUE,

            accept_ex_fn: unsafe { mem::zeroed() },
            get_accept_sock_addrs_fn: unsafe { mem::zeroed() },
        }
    }
}

impl Drop for MainContext  {
    fn drop(&mut self) {
        unsafe {
            closesocket(self.socket_fd);
            CloseHandle(self.iocp);
            WSACleanup();
        };
    }
}


pub struct IOContext 
{
    pub over_lapped: OVERLAPPED,
    pub accept_fd: SOCKET,
    pub action: usize,
    pub buf: [i8; BUFF_SIZE as usize],
}

impl IOContext {

    pub fn new() -> Self {
        IOContext {
            over_lapped: unsafe { mem::zeroed() },
            accept_fd: INVALID_SOCKET,
            buf: unsafe { mem::zeroed() },
            action: 10,
        }
    }
}




fn main() {

    let std_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9120);


    let mut ctx = MainContext::new();
    let ctx_ptr = &mut ctx as *mut MainContext;


    load_wsa();


    ctx.iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
    ctx.socket_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };


    println!("IOCP: {:?}, Socket: {:?}", ctx.iocp, ctx.socket_fd);


    let worker = unsafe { CreateThread(ptr::null_mut(), 0, Some(worker), ctx_ptr as *mut _, 0, 0 as *mut _)};


    // Associate the listening socket with the completion port
    unsafe { CreateIoCompletionPort(ctx.socket_fd as HANDLE, ctx.iocp, ctx_ptr as usize, 0) };


    let (addr, addr_len) = socket_addr_to_ptrs(&std_addr);
    let bind_ret = unsafe { bind(ctx.socket_fd, addr, addr_len) };

    println!("bind: {:?}  WSAGetLastError: {:?}", bind_ret, unsafe { WSAGetLastError() });

    unsafe { 
        let mut reuse: c_char = 1;
        setsockopt(ctx.socket_fd, SOL_SOCKET, SO_REUSEADDR, &mut reuse as *const c_char, mem::size_of::<c_int>() as c_int)
    };
    unsafe { listen(ctx.socket_fd, SOMAXCONN) };


    ctx.accept_ex_fn = accept_ex_ref(ctx.socket_fd);
    ctx.get_accept_sock_addrs_fn = get_accept_sock_addrs_ref(ctx.socket_fd);


    println!("WSAGetLastError: {:?}", unsafe { WSAGetLastError() });

    
    post_accept(&ctx);


    unsafe {
        WaitForSingleObject(worker, INFINITE)
    };

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


fn accept_ex_ref(socket_fd: usize) -> LPFN_AcceptEx {

    let mut accept_ex_fn = 0 as usize;
    let mut bytes = 0;

    let io_ret = unsafe {
        WSAIoctl(
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &WSAID_ACCEPTEX as *const _ as *mut _,
            mem::size_of_val(&WSAID_ACCEPTEX) as DWORD,
            &mut accept_ex_fn as *mut _ as *mut c_void,
            mem::size_of_val(&accept_ex_fn) as DWORD,
            &mut bytes,
            0 as *mut _, 
            None)
    };

    if io_ret == SOCKET_ERROR {
        println!("WSAIoctl(LPFN_AcceptEx) failed.");
    }
    
    unsafe { mem::transmute::<_, LPFN_AcceptEx>(accept_ex_fn) }
}


fn get_accept_sock_addrs_ref(socket_fd: usize) -> LPFN_GetAcceptExSockaddrs {

    let mut bytes = 0;
    let mut get_accept_sock_addrs_fn = 0 as usize;
    
    let io_ret = unsafe {
        WSAIoctl(
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &WSAID_GETACCEPTEXSOCKADDRS as *const _ as *mut _,
            mem::size_of_val(&WSAID_GETACCEPTEXSOCKADDRS) as DWORD,
            &mut get_accept_sock_addrs_fn as *mut _ as *mut c_void,
            mem::size_of_val(&get_accept_sock_addrs_fn) as DWORD,
            &mut bytes,
            0 as *mut _, 
            None)
    };

    if io_ret == SOCKET_ERROR {
        println!("WSAIoctl(SIO_GET_EXTENSION_FUNCTION_POINTER) failed.");
    }

    
    unsafe { mem::transmute::<_, LPFN_GetAcceptExSockaddrs>(get_accept_sock_addrs_fn) }
}


unsafe extern "system" fn worker(param: LPVOID) -> u32 {
    println!("Worker :{:?}", param);

    let ctx = mem::transmute::<_, &mut MainContext>(param);

    let mut count = 0;
    let mut action_type = 10;
    let mut over_lapped = mem::zeroed();

    loop {
        GetQueuedCompletionStatus(
            ctx.iocp,
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


fn post_accept(ctx: &MainContext) {

    let mut io_ctx = IOContext::new();
    io_ctx.action = 0;
    io_ctx.accept_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };


    let sock_len = mem::size_of::<SOCKADDR_IN>() as u32;
    let mut dw_bytes = 0;

    unsafe {
        (ctx.accept_ex_fn)(
            ctx.socket_fd,
            io_ctx.accept_fd,
            &mut io_ctx.buf as *mut _ as *mut c_void,
            0,
            sock_len + 16,
            sock_len + 16,
            &mut dw_bytes,
            &mut io_ctx.over_lapped
        )
    };
    

    println!("post_accept, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
}


fn do_accept() {

}

fn do_recv() {

}

fn do_send() {}