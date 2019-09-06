#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]


use std::{ptr, mem, cmp};
use std::os::raw::{c_int, c_char, c_ushort, c_ulong};
use std::net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr};
pub type sa_family_t = c_ushort;
pub type socklen_t = c_int;

use winapi::ctypes::c_void;
use winapi::um::winnt::HANDLE;
use winapi::shared::guiddef::GUID;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::shared::ws2def::{AF_INET, SOCK_STREAM, SOCKADDR, SOCKADDR_IN, IPPROTO_IP, SIO_GET_EXTENSION_FUNCTION_POINTER, WSABUF};
use winapi::um::winsock2::{u_long, SOCKET, WSADATA, LPWSADATA, SOL_SOCKET, SO_REUSEADDR, SOCKET_ERROR, INVALID_SOCKET, SOMAXCONN, WSA_FLAG_OVERLAPPED, WSADESCRIPTION_LEN, WSASYS_STATUS_LEN};
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
use winapi::um::synchapi::WaitForSingleObject;


type LPFN_AcceptEx = unsafe extern "system" fn(SOCKET, SOCKET, PVOID, DWORD, DWORD, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;
type LPFN_GetAcceptExSockaddrs = unsafe extern "system" fn(PVOID, DWORD, DWORD, DWORD, *mut SOCKADDR, *mut c_int, *mut SOCKADDR, *mut c_int);


const MAX_WORKERS: u32 = 1;
const MAX_BUFFER_LEN: u32 = 4096;


struct Context {
    pub iocp: HANDLE,
    pub listen_fd: SOCKET,

    pub workers: Vec<HANDLE>,

    pub accept_ex_fn: LPFN_AcceptEx,
    pub get_accept_sock_addrs_fn: LPFN_GetAcceptExSockaddrs,
}


struct SocketContext {
    pub socket: SOCKET,                     // Socket
}


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
            workers: Vec::new(),
            accept_ex_fn: unsafe { mem::zeroed() },
            get_accept_sock_addrs_fn: unsafe { mem::zeroed() },
        }
    }
}

impl SocketContext {
    pub fn new() -> Self {
        SocketContext {
            socket: INVALID_SOCKET
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



fn main() {
    let mut ctx = Context::new();
    let ctx_ptr = &mut ctx as *mut Context;
    let std_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000);


    load_wsa();


    ctx.iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
    ctx.listen_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };


    println!("Context: {:?}, IOCP: {:?}, Socket: {:?}", ctx_ptr, ctx.iocp, ctx.listen_fd);

    
    for _ in 0..MAX_WORKERS {
        let thandel = unsafe { CreateThread(ptr::null_mut(), 0, Some(worker), ctx_ptr as *mut _, 0, 0 as *mut _)};
        ctx.workers.push(thandel);
    }

    println!("Worker thread: {:?}", ctx.workers[0]);


    let mut socket_ctx = SocketContext::new();
    socket_ctx.socket = ctx.listen_fd;

    println!("SocketContext: {:?}", &socket_ctx as *const _);

    // Associate the listening socket with the completion port
    unsafe { CreateIoCompletionPort(ctx.listen_fd as HANDLE, ctx.iocp, &mut socket_ctx as *mut _ as usize, 0) };



    unsafe { 
        let mut reuse: c_char = 1;
        setsockopt(ctx.listen_fd, SOL_SOCKET, SO_REUSEADDR, &mut reuse as *const c_char, mem::size_of::<c_int>() as c_int)
    };

    let (addr, addr_len) = socket_addr_to_ptrs(&std_addr);
    let bind_ret = unsafe { bind(ctx.listen_fd, addr as _, addr_len) };

    println!("bind: {:?}  WSAGetLastError: {:?}", bind_ret, unsafe { WSAGetLastError() });

    unsafe { listen(ctx.listen_fd, SOMAXCONN) };


    ctx.accept_ex_fn = accept_ex_ref(ctx.listen_fd);
    ctx.get_accept_sock_addrs_fn = get_accept_sock_addrs_ref(ctx.listen_fd);


    println!("WSAGetLastError: {:?}", unsafe { WSAGetLastError() });

    
    for _ in 0..MAX_WORKERS {
        let mut io_ctx = PerIOContext::new();
        println!("PerIOContext: {:?}", &io_ctx as *const _);

        post_accept(&ctx, &socket_ctx, &mut io_ctx);
    }


    println!("Worker handle: {:?} WSAGetLastError: {:?}", ctx.workers[0], unsafe { WSAGetLastError() });


    unsafe {
        WaitForSingleObject(ctx.workers[0], INFINITE)
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

        let mut count = 0;
        let mut socket_ptr = mem::zeroed();
        let mut io_ctx_ptr = mem::zeroed();

        GetQueuedCompletionStatus(
            ctx.iocp,
            &mut count,
            &mut socket_ptr,
            &mut io_ctx_ptr,
            INFINITE
        );

        println!("Have a QueuedCompletionStatus");

        let socket_ctx = mem::transmute::<_, &mut SocketContext>(socket_ptr);
        let mut io_ctx = mem::transmute::<_, &mut PerIOContext>(io_ctx_ptr);

        println!("Action: {:?}", io_ctx.action);

        match io_ctx.action {
            0 => do_accept(&ctx, &socket_ctx, &mut io_ctx),
            1 => do_recv(&socket_ctx, &mut io_ctx),
            2 => do_send(&socket_ctx, &mut io_ctx),
            _ => println!("Error: {:?}", WSAGetLastError()),
        }
    }
}


fn post_accept(ctx: &Context, socket_ctx: &SocketContext, io_ctx: &mut PerIOContext) {
    println!("post_accept");

    
    let sock_len = mem::size_of::<SOCKADDR_IN>() as u32;
    io_ctx.accept_fd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, IPPROTO_IP, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };
    io_ctx.action = 0;


    unsafe {
        (ctx.accept_ex_fn)(
            socket_ctx.socket,
            io_ctx.accept_fd,

            &mut io_ctx.wsa_buf.buf as *mut _ as *mut c_void,
            0,

            sock_len + 16,
            sock_len + 16,

            &mut io_ctx.recv_bytes,
            &mut io_ctx.ol
        )
    };

    println!("post_accept, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
}


fn do_accept(ctx: &Context, socket_ctx: &SocketContext, io_ctx: &mut PerIOContext) {
    println!("do_accept");

    /*
    unsafe { 
        setsockopt(
            io_ctx.accept_fd, 
            SOL_SOCKET, 
            SO_UPDATE_ACCEPT_CONTEXT, 
            ctx.listen_fd as *const _, 
            mem::size_of_val(&ctx.listen_fd) as c_int)
    };

    println!("setsockopt(SO_UPDATE_ACCEPT_CONTEXT)");
    */
    
    /*
    let mut locale_addr_ptr = unsafe { mem::zeroed() };
    let mut client_addr_ptr = unsafe { mem::zeroed() };
    let mut add_len = mem::size_of::<SOCKADDR_IN>() as u32;

    unsafe {
        (ctx.get_accept_sock_addrs_fn)(
            io_ctx.wsa_buf.buf as *mut _,
            io_ctx.wsa_buf.len - (add_len + 16) * 2,

            add_len + 16,
            add_len + 16,

            &mut locale_addr_ptr as *mut _,
            &mut add_len as *mut _ as *mut c_int,
            &mut client_addr_ptr as *mut _,
            &mut add_len as *mut _ as *mut c_int,
        )
    };

    
    let locale_addr = unsafe { mem::transmute::<_, *mut std::net::SocketAddrV4>(locale_addr_ptr) };
    let client_addr = unsafe { mem::transmute::<_, *mut std::net::SocketAddrV4>(client_addr_ptr) };
    
    
    println!("Locale addr: {:?}", locale_addr_ptr);
    println!("Client addr: {:?}", client_addr_ptr);
    */
    
    
    let mut new_socket_ctx = SocketContext::new();
    new_socket_ctx.socket = io_ctx.accept_fd;

    println!("do_accept, client_socket: {:?}", io_ctx.accept_fd);


    io_ctx.reset();
    post_accept(&ctx, &socket_ctx, io_ctx);

    
    unsafe { 
        CreateIoCompletionPort(
            new_socket_ctx.socket as HANDLE, 
            ctx.iocp, 
            &mut new_socket_ctx as *mut _ as usize, 
            0) 
    };

    println!("do_accept, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });


    let mut new_io_ctx = PerIOContext::new();
    new_io_ctx.accept_fd = new_socket_ctx.socket;

    post_recv(&mut new_io_ctx);
}


fn post_recv(io_ctx: &mut PerIOContext) {
    println!("post_recv, socket: {:?}", io_ctx.accept_fd);

    let mut dw_byetes = 0;
    let mut dw_flags = 0;

    io_ctx.action = 1;
    
    unsafe { 
        WSARecv(
            io_ctx.accept_fd, 
            &mut io_ctx.wsa_buf, 
            1, 
            &mut dw_byetes, 
            &mut dw_flags, 
            &mut io_ctx.ol, 
            None)
    };

    println!("post_recv, WSAGetLastError: {:?}", unsafe { WSAGetLastError() });
}


fn do_recv(ctx: &SocketContext, io_ctx: &mut PerIOContext) {
    println!("do_recv");
    println!("Recv data: {:?}", io_ctx.wsa_buf.buf);

    post_send(&ctx, io_ctx);
}


fn post_send(ctx: &SocketContext, io_ctx: &mut PerIOContext) {
    println!("post_send");

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nWelcome to Server".as_bytes();

    io_ctx.reset();
    io_ctx.wsa_buf = unsafe { slice2buf(&response) };
    io_ctx.action = 2;

    unsafe {
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

}

fn do_send(ctx: &SocketContext, io_ctx: &PerIOContext) {
    println!("do_send");
}