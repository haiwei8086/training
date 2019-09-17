#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


use std::{ptr, mem, cmp};
use std::ffi::CStr;
use std::os::raw::{c_int, c_char, c_ushort, c_ulong};
use std::net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr};
pub type sa_family_t = c_ushort;
pub type socklen_t = c_int;

use winapi::ctypes::c_void;
use winapi::um::winnt::HANDLE;
use winapi::shared::guiddef::GUID;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::ws2def::{AF_INET, SOCK_STREAM, SOCKADDR, SOCKADDR_IN, IPPROTO_IP, SIO_GET_EXTENSION_FUNCTION_POINTER, WSABUF};
use winapi::um::winsock2::{u_long, SOCKET, WSADATA, LPWSADATA, SD_BOTH, SOL_SOCKET, SO_REUSEADDR, SO_KEEPALIVE, SOCKET_ERROR, INVALID_SOCKET, SOMAXCONN, WSA_IO_PENDING, WSA_FLAG_OVERLAPPED, WSADESCRIPTION_LEN, WSASYS_STATUS_LEN};
use winapi::um::winnt::PVOID;
use winapi::shared::minwindef::{DWORD, BOOL, LPVOID, LPDWORD};
use winapi::um::minwinbase::{OVERLAPPED, LPOVERLAPPED, OVERLAPPED_ENTRY};
use winapi::um::mswsock::{SO_UPDATE_ACCEPT_CONTEXT};
use winapi::um::winbase::INFINITE;
use winapi::shared::ws2ipdef::SOCKADDR_IN6_LH;
use winapi::um::mswsock::{WSAID_ACCEPTEX, WSAID_GETACCEPTEXSOCKADDRS};


use winapi::um::winsock2::{WSAIoctl, WSAGetLastError, WSAStartup, WSACleanup, WSASocketW, WSARecv, WSASend, socket, setsockopt, bind, listen, closesocket, shutdown, htons, inet_addr};
use winapi::shared::minwindef::{LOBYTE, HIBYTE};
use winapi::um::ioapiset::{CreateIoCompletionPort, GetQueuedCompletionStatus};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::{WaitForMultipleObjects, WaitForSingleObject};


type LPFN_AcceptEx = unsafe extern "system" fn(SOCKET, SOCKET, PVOID, DWORD, DWORD, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;
type LPFN_GetAcceptExSockaddrs = unsafe extern "system" fn(PVOID, DWORD, DWORD, DWORD, *mut SOCKADDR, *mut c_int, *mut SOCKADDR, *mut c_int);

const PORT: u16 = 5001;
const MAX_WORKERS: u32 = 2;
const MAX_BUFFER_LEN: u32 = 4096;

pub struct SignalEvent {
    pub name: &'static str,
    pub path_format: &'static str,
    pub path: String,
    pub handle: HANDLE,
}


struct Context {
    pub iocp: HANDLE,
    pub listen_fd: SOCKET,

    pub workers: [HANDLE; MAX_WORKERS as usize],

    pub accept_ex_fn: LPFN_AcceptEx,
    pub get_accept_sock_addrs_fn: LPFN_GetAcceptExSockaddrs,
}


struct ClientContent {
    pub socket_fd: SOCKET,
}


#[derive(Clone)]
struct PerIOContext {
    pub ol: OVERLAPPED,                     // 每一个重叠I/O网络操作都要有一个
    pub accept_fd: SOCKET,                  // 这个I/O操作所使用的Socket，每个连接的都是一样的
    pub wsa_buf: WSABUF,                    // 存储数据的缓冲区，用来给重叠操作传递参数的，关于WSABUF后面
    pub buf: [i8; MAX_BUFFER_LEN as usize], // 真正接收数据得buffer
    pub recv_bytes: u32,                    // 接收的数量
    pub send_bytes: u32,                    // 发送的数量
    pub action: usize,                      // 标志这个重叠I/O操作是做什么的，例如Accept/Recv等
}


impl Context {
    pub fn new() -> Self {
        Context {
            iocp: INVALID_HANDLE_VALUE,
            listen_fd: INVALID_SOCKET,
            workers: unsafe { mem::zeroed() },
            accept_ex_fn: unsafe { mem::zeroed() },
            get_accept_sock_addrs_fn: unsafe { mem::zeroed() },
        }
    }
}

impl ClientContent {
    pub fn new() -> Self {
        ClientContent {
            socket_fd: INVALID_SOCKET,
        }
    }
}

impl PerIOContext {
    pub fn new() -> Self {
        let mut ctx = PerIOContext {
            ol: unsafe { mem::zeroed() },
            accept_fd: INVALID_SOCKET,
            wsa_buf: unsafe { mem::zeroed() },
            buf: unsafe { mem::zeroed() },
            recv_bytes: 0,
            send_bytes: 0,
            action: 10,
        };

        ctx.wsa_buf = WSABUF {
            len: MAX_BUFFER_LEN,
            buf: ctx.buf.as_ptr() as *mut _,
        };

        ctx
    }

    pub fn reset(&mut self) {
        self.ol = unsafe { mem::zeroed() };
        self.buf = unsafe { mem::zeroed() };
        self.action = 10;
        self.recv_bytes = 0;
        self.send_bytes = 0;

        self.wsa_buf = WSABUF {
            len: MAX_BUFFER_LEN,
            buf: self.buf.as_ptr() as *mut _,
        };
    }
}

impl Drop for PerIOContext {
    fn drop(&mut self) {
        println!("io ctx drop. socket: {:?}", self.accept_fd);

        //unsafe { shutdown(self.accept_fd, SD_BOTH) };
    }
}



fn main() {
    let mut ctx = Context::new();
    let ctx_ptr = &mut ctx as *mut Context;
    let std_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);


    let mut client_ctx = ClientContent::new();
    let client_ctx_ptr = &mut client_ctx as *mut ClientContent;

    load_wsa();


    ctx.iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
    ctx.listen_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };

    println!("Context: {:?}, IOCP: {:?}, Socket: {:?}", ctx_ptr, ctx.iocp, ctx.listen_fd);

    unsafe { setsockopt(ctx.listen_fd, SOL_SOCKET, SO_REUSEADDR, 1 as *const c_char, mem::size_of::<c_int>() as c_int) };
    unsafe { setsockopt(ctx.listen_fd, SOL_SOCKET, SO_KEEPALIVE, 1 as *const c_char, mem::size_of::<c_int>() as c_int) };


    let mut work_thread: HANDLE = INVALID_HANDLE_VALUE;
    let mut thread_id:u32 = 0;

    for i in 0..MAX_WORKERS {
        work_thread = unsafe { CreateThread(ptr::null_mut(), 0, Some(worker), ctx_ptr as *mut _, 0, &mut thread_id as *mut _)};
        ctx.workers[i as usize] = work_thread;
        println!("new thread: {:?}", ctx.workers[i as usize]);
    }

    // Associate the listening socket with the completion port
    unsafe { CreateIoCompletionPort(ctx.listen_fd as HANDLE, ctx.iocp, client_ctx_ptr as usize, 0) };


    let (addr, addr_len) = socket_addr_to_ptrs(&std_addr);
    unsafe { bind(ctx.listen_fd, addr as _, addr_len) };
    unsafe { listen(ctx.listen_fd, SOMAXCONN) };

    println!("Listen on port({})", PORT);

    ctx.accept_ex_fn = accept_ex_ref(ctx.listen_fd);
    ctx.get_accept_sock_addrs_fn = get_accept_sock_addrs_ref(ctx.listen_fd);

    
    let mut io_ctx_list = vec![PerIOContext::new(); MAX_WORKERS as usize];

    for i in 0..MAX_WORKERS {
        post_accept_ex(&ctx, &mut io_ctx_list[i as usize]);
    }

    unsafe {
        WaitForSingleObject(work_thread, INFINITE)
        //WaitForMultipleObjects(ctx.workers.len() as DWORD, ctx.workers.as_ptr(), 0, INFINITE)
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


fn socket_addr_to_ptrs(addr: &SocketAddr) -> (*const SOCKADDR_IN, c_int) {
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
        println!("WSAIoctl(LPFN_GetAcceptExSockaddrs) failed.");
    }

    
    unsafe { mem::transmute::<_, LPFN_GetAcceptExSockaddrs>(get_accept_sock_addrs_fn) }
}


unsafe fn slice2buf(slice: &[u8]) -> WSABUF {
    WSABUF {
        len: cmp::min(slice.len(), <u_long>::max_value() as usize) as u_long,
        buf: slice.as_ptr() as *mut _,
    }
}


unsafe extern "system" fn worker(param: LPVOID) -> u32 {
    let ctx = mem::transmute::<_, &mut Context>(param);

    println!("Worker Context: {:?}, IOCP: {:?}, Socket: {:?}", param, ctx.iocp, ctx.listen_fd);

    loop {
        println!("GetQueuedCompletionStatus...");

        let mut bytes = 0;
        let mut client_ctx_ptr = mem::zeroed();
        let mut io_ctx_ptr = mem::zeroed();

        GetQueuedCompletionStatus(
            ctx.iocp,
            &mut bytes,
            &mut client_ctx_ptr as *mut _,
            &mut io_ctx_ptr as *mut _,
            INFINITE
        );

        println!("QueuedCompletionStatus, client_ctx: {:?}, io_ctx: {:?}", client_ctx_ptr, io_ctx_ptr);

        let mut io_ctx = mem::transmute::<_, &mut PerIOContext>(io_ctx_ptr);
        let mut new_io_ctx = PerIOContext::new();
        new_io_ctx.action = 20;

        println!("Action: {:?}, new_io: {:p}", io_ctx.action, &mut new_io_ctx as *mut _);


        match io_ctx.action {
            0 => {
                post_accept_ex(&ctx, &mut new_io_ctx);
                do_accept(&ctx, &mut io_ctx);
                post_recv(&mut io_ctx);
            },
            1 => {
                do_recv(&io_ctx);
                post_send(&mut io_ctx);
            },
            2 => do_send(&io_ctx),
            _ => println!("Error: {:?}", WSAGetLastError()),
        }
    }
}


fn post_accept_ex(ctx: &Context, io_ctx: &mut PerIOContext) {
    println!("post_accept_ex, io_ctx action: {:?}", io_ctx.action);


    let sock_len = mem::size_of::<SOCKADDR_IN>() as u32;

    io_ctx.accept_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };
    io_ctx.action = 0;

    println!("post_accept_ex, ctx: {:?}, listener: {:?}, io_ctx: {:p}", ctx as *const _, ctx.listen_fd, &io_ctx as *const _);

    unsafe {
        (ctx.accept_ex_fn)(
            ctx.listen_fd,
            io_ctx.accept_fd,

            &mut io_ctx.wsa_buf.buf as *mut _ as *mut _,
            0,

            sock_len + 16,
            sock_len + 16,

            &mut io_ctx.recv_bytes,
            &mut io_ctx.ol
        )
    };


    let err = unsafe { WSAGetLastError() };

    if WSA_IO_PENDING != err {
        println!("accept_ex_fn() failed, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
    }
}


fn do_accept(ctx: &Context, io_ctx: &mut PerIOContext) {
    println!("do_accept, ctx: {:?}, io_ctx: {:?}, client: {:?}", ctx as *const _, io_ctx as *const _, io_ctx.accept_fd);

    unsafe { 
        setsockopt(
            io_ctx.accept_fd, 
            SOL_SOCKET, 
            SO_UPDATE_ACCEPT_CONTEXT, 
            &ctx.listen_fd as *const _ as *const _, 
            mem::size_of::<SOCKET>() as c_int)
    };

    println!("setsockopt(SO_UPDATE_ACCEPT_CONTEXT), WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
    
    /*
    let mut locale_addr_ptr: SOCKADDR_IN = unsafe { mem::zeroed() };
    let mut client_addr_ptr: SOCKADDR_IN = unsafe { mem::zeroed() };
    let mut add_len = mem::size_of::<SOCKADDR_IN>() as u32;

    unsafe {
        (ctx.get_accept_sock_addrs_fn)(
            io_ctx.wsa_buf.buf as *mut _,
            io_ctx.wsa_buf.len - (add_len + 16) * 2,

            add_len + 16,
            add_len + 16,

            &mut locale_addr_ptr as *mut _ as *mut _,
            &mut add_len as *mut _ as *mut c_int,
            &mut client_addr_ptr as *mut _ as *mut _,
            &mut add_len as *mut _ as *mut c_int,
        )
    };
    */

    /*    
    let locale_addr = unsafe { mem::transmute::<_, *mut std::net::SocketAddrV4>(locale_addr_ptr) };
    let client_addr = unsafe { mem::transmute::<_, *mut std::net::SocketAddrV4>(client_addr_ptr) };
    
    
    println!("Locale addr: {:?}", locale_addr_ptr);
    println!("Client addr: {:?}", client_addr_ptr);
    */
    
    let mut client_ctx = ClientContent::new();
    let client_ctx_ptr = &mut client_ctx as *mut ClientContent;

    unsafe { 
        CreateIoCompletionPort(
            io_ctx.accept_fd as HANDLE, 
            ctx.iocp, 
            client_ctx_ptr as usize, 
            0) 
    };

    println!("do_accept done, ctx: {:?}, io_ctx: {:?}, client: {:?}", ctx as *const _, io_ctx as *const _, io_ctx.accept_fd);
}


fn post_recv(io_ctx: &mut PerIOContext) {
    println!("post_recv, io_ctx: {:?}, client: {:?}", io_ctx as *const _, io_ctx.accept_fd);

    let mut byetes = 0;
    let mut flags = 0;

    io_ctx.reset();
    io_ctx.action = 1;
    
    let ret = unsafe { 
        WSARecv(
            io_ctx.accept_fd, 
            &mut io_ctx.wsa_buf, 
            1, 
            &mut byetes, 
            &mut flags, 
            &mut io_ctx.ol, 
            None)
    };

    if -1 == ret {
        let err = unsafe { WSAGetLastError() };

        if WSA_IO_PENDING != err {
            println!("WSARecv() failed, WSAGetLastError: {:?}", err);
        }
    }

    println!("post_recv done. WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
}


fn do_recv(io_ctx: &PerIOContext) {
    println!("do_recv");
    println!("Recv data: {:?}", unsafe { CStr::from_ptr(io_ctx.wsa_buf.buf) });
}


fn post_send(io_ctx: &mut PerIOContext) {
    println!("post_send, io_ctx: {:?}, client: {:?}", io_ctx as *const _, io_ctx.accept_fd);

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nWelcome to Server.".as_bytes();

    io_ctx.reset();
    io_ctx.wsa_buf = unsafe { slice2buf(&response) };
    io_ctx.action = 2;

    let ret = unsafe {
        WSASend(
            io_ctx.accept_fd,
            &mut io_ctx.wsa_buf as *mut _,
            1,
            &mut io_ctx.send_bytes as *mut _,
            0,
            &mut io_ctx.ol,
            None
        )
    };

    if -1 == ret {
        let err = unsafe { WSAGetLastError() };

        if WSA_IO_PENDING != err {
            println!("WSASend() failed, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
        }
    }
}

fn do_send(io_ctx: &PerIOContext) {
    println!("do_send, io_ctx: {:?}, client: {:?}", io_ctx as *const _, io_ctx.accept_fd);

    unsafe { shutdown(io_ctx.accept_fd, SD_BOTH) };
}