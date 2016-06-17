extern crate winapi;
extern crate kernel32;
extern crate ws2_32;

mod iocp;

use self::winapi::{c_void, WSADATA};
use self::winapi::winnt::{PVOID, HANDLE, RTL_CRITICAL_SECTION};
use self::winapi::minwinbase::{OVERLAPPED};
use self::winapi::sysinfoapi::SYSTEM_INFO;
use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::minwindef::{LPVOID, HIBYTE, LOBYTE};
use self::winapi::winsock2::{LPWSADATA, WSADESCRIPTION_LEN, WSASYS_STATUS_LEN, INVALID_SOCKET, SOMAXCONN};
use self::winapi::inaddr::IN_ADDR;
use self::kernel32::{GetSystemInfo, CreateIoCompletionPort};
use self::winapi::ws2def::*;
use self::ws2_32::*;

use std::{mem, ptr};

const MAX_POST_ACCEPT: u32 = 10;
const WSA_FLAG_OVERLAPPED: u32 = 0x01;
const MAX_BUFFER_LEN: usize = 4096;

enum OperationType {
    ACCEPT,
    SEND,
    RECV,
    NULL,
}

struct PerIOContext {
    pub m_overlapped: OVERLAPPED,              // 每一个重叠I/O网络操作都要有一个
    pub m_socket_accept: u64,                  // 这个I/O操作所使用的Socket，每个连接的都是一样的
    pub m_wsa_buf: WSABUF,                     // 存储数据的缓冲区，用来给重叠操作传递参数的，关于WSABUF后面
    pub m_sz_buffer: [char; MAX_BUFFER_LEN],   // 对应WSABUF里的缓冲区
    pub m_op: OperationType,                   // 标志这个重叠I/O操作是做什么的，例如Accept/Recv等
}

impl PerIOContext {

    pub fn new() -> PerIOContext {
        PerIOContext {
            m_overlapped: OVERLAPPED {
                Internal: 0,
                InternalHigh: 0,
                Offset: 0,
                OffsetHigh: 0,
                hEvent: ptr::null_mut(),
            },
            m_socket_accept: 0,
            m_wsa_buf: WSABUF {
                len: MAX_BUFFER_LEN as u32,
                buf: ptr::null_mut(),
            },
            m_sz_buffer: ['0'; MAX_BUFFER_LEN],
            m_op: OperationType::NULL,
        }
    }
}

struct PerSocketContext {
    pub socket: u64,                       // 每一个客户端连接的Socket
    pub client_addr: SOCKADDR_IN,          // 这个客户端的地址
    pub io_contexts: Vec<PerIOContext>,   // 数组，所有客户端IO操作的参数，
}

impl PerSocketContext {

    pub fn new() -> PerSocketContext {
        PerSocketContext {
            socket: INVALID_SOCKET,
            client_addr: unsafe { mem::transmute(*ptr::null::<SOCKADDR_IN>()) },
            io_contexts: Vec::new(),
        }
    }
}

struct WorkerThreadParams {
    iocp: HANDLE,
    number: u32,
}


pub fn run() {

    let mut wio = iocp::WinIOCP::new();
    wio.run();
}

fn mod_run() {
    // 初始化线程互斥量
    let mut critical_section = unsafe { mem::zeroed::<RTL_CRITICAL_SECTION>() };
    unsafe { kernel32::InitializeCriticalSection(&mut critical_section) };

    let wsa = load_wsa().unwrap();
    println!("WSA start up: {:?}", wsa);

    // 初始化IOCP
    let mut iocp = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
    if iocp == ptr::null_mut() {
        println!("Create IOCP Failed!");
        return;
    }
    println!("Create IOCP: {:?}", iocp);

    let mut threads: Vec<HANDLE> = Vec::new();
    for i in 0..2 {
        let mut thread_no: u32 = i + 1;
        threads.push(
            unsafe {
                kernel32::CreateThread(
                    ptr::null_mut(),
                    0,
                    Some(worker),
                    iocp,
                    0,
                    &mut thread_no
                )
            }
        );
    }

    let info = get_system_info();
    println!("System processors: {:?}", info.dwNumberOfProcessors);

    let sockfd = listen_socket().unwrap();

    println!("Win IOCP run...");
}

fn load_wsa() -> Result<i32, i32> {
    let mut vendor = 0i8;

    let mut wsa_data = WSADATA {
        wVersion: 0,
        wHighVersion: 0,
        iMaxSockets: 0,
        iMaxUdpDg: 0,
        lpVendorInfo: &mut vendor,
        szDescription: [0i8; WSADESCRIPTION_LEN + 1],
        szSystemStatus: [0i8; WSASYS_STATUS_LEN + 1],
    };

    let ret = unsafe { WSAStartup(0x0202, &mut wsa_data as LPWSADATA) };

    if LOBYTE(wsa_data.wVersion) != 2 && HIBYTE(wsa_data.wVersion) != 2 {

        unsafe { WSACleanup() };

        return Err(0);
    }

    println!("WinSock version: {:?}, {:?}", LOBYTE(wsa_data.wVersion), HIBYTE(wsa_data.wVersion));

    return if ret < 0 { Err(ret) } else { Ok(ret) };
}

fn get_system_info() -> SYSTEM_INFO {
    let mut info = SYSTEM_INFO {
        wProcessorArchitecture: 0,
        wReserved: 0,
        dwPageSize: 0,
        lpMinimumApplicationAddress: ptr::null_mut(),
        lpMaximumApplicationAddress: ptr::null_mut(),
        dwActiveProcessorMask: 0,
        dwNumberOfProcessors: 0,
        dwProcessorType: 0,
        dwAllocationGranularity: 0,
        wProcessorLevel: 0,
        wProcessorRevision: 0,
    };

    unsafe { GetSystemInfo(&mut info) };

    // println!("System info: {:?}", info);
    info
}

fn get_addr_len() -> (SOCKADDR, i32) {
    let addr = SOCKADDR_IN {
            sin_family: AF_INET as u16,
            sin_port: unsafe{ htons(9000) },
            sin_addr: IN_ADDR {
                S_un: 0 as u32,
            },
            .. unsafe { mem::zeroed() }
        };
    (unsafe {mem::transmute(addr)}, mem::size_of::<SOCKADDR_IN>() as i32)
}

fn listen_socket() -> Result<u64, i32> {

    let sockfd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };
    println!("Create socket: {:?}", sockfd);

    if sockfd == INVALID_SOCKET {
        println!("Invalid socket! {:?}", sockfd);
        return Err(-1);
    }

    let (addr, addr_len) = get_addr_len();
    println!("Socket addr: {:?} len: {:?}", addr, addr_len);

    if -1 == unsafe { bind(sockfd, &addr as *const SOCKADDR, addr_len) } {
        println!("Bind socket failed!");
        return Err(-1);
    }
    println!("Bind socket successed!");

    if -1 == unsafe { listen(sockfd, SOMAXCONN) } {
        println!("Socket listen failed!");
        return Err(-1);
    }
    println!("Listen socket successed!");

    Ok(sockfd)
}

unsafe extern "system" fn worker(param: LPVOID) -> u32 {
    println!("Worker :{:?}", param);

    /*
    loop {
        let mut count = 0;
        let mut socket_fd = INVALID_SOCKET;
        let mut over_lapped = ptr::null_mut::<LPOVERLAPPED>();

        let status = unsafe {
            kernel32::GetQueuedCompletionStatus(
                param,
                &mut count,
                &mut socket_fd,
                over_lapped,
                INFINITE
            )
        };

        println!("Thread socket:{:?}", socket_fd);

    }
    */

    0
}
