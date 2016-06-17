
extern crate winapi;
extern crate kernel32;
extern crate ws2_32;

use self::winapi::{c_void, WSADATA};
use self::winapi::winnt::{PVOID, HANDLE, RTL_CRITICAL_SECTION};
use self::winapi::winbase::INFINITE;
use self::winapi::minwinbase::{OVERLAPPED, LPOVERLAPPED};
use self::winapi::sysinfoapi::SYSTEM_INFO;
use self::winapi::shlobj::INVALID_HANDLE_VALUE;
use self::winapi::minwindef::{LPVOID, HIBYTE, LOBYTE};
use self::winapi::winsock2::{LPWSADATA, WSADESCRIPTION_LEN, WSASYS_STATUS_LEN, INVALID_SOCKET, SOMAXCONN};
use self::winapi::inaddr::IN_ADDR;
use self::winapi::ws2def::*;
use self::ws2_32::*;


use std::{mem, ptr, thread, rc};
use std::os::raw;
use std::sync::{Arc, Mutex, mpsc};
use std::marker::{Sync, Send};
use std::ops::Deref;

const WSA_FLAG_OVERLAPPED: u32 = 0x01;
const MAX_BUFFER_LEN: usize = 4096;


// 操作类型
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
}

impl PerSocketContext {

    pub fn new() -> PerSocketContext {
        PerSocketContext {
            socket: INVALID_SOCKET,
            client_addr: unsafe { mem::transmute(*ptr::null::<SOCKADDR_IN>()) },
        }
    }
}

pub struct WinIOCP {
    pub iocp: HANDLE,
}

unsafe impl Send for WinIOCP {}

impl WinIOCP {

    pub fn new() -> WinIOCP {
        WinIOCP {
            iocp: unsafe { mem::zeroed() },
        }
    }

    pub fn run(&mut self) {
        println!("Win IOCP running...");

        WinIOCP::load_wsa().unwrap();
        WinIOCP::get_sys_info();

        self.InitializeIOCP();
        self.InitializeListen().unwrap();

    }

    fn InitializeIOCP(&mut self) {
        let iocp = unsafe { kernel32::CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) };
        if iocp == ptr::null_mut() {
            println!("Initialize IOCP failed!");
            return;
        }
        println!("Initialize IOCP successed! : {:?}", iocp);

        self.iocp = iocp;

        let this : WinIOCP = unsafe { mem::transmute(self) };

        let params = Arc::new(Mutex::new(this));
        let (tx, rx) = mpsc::channel();

        for i in 0..2 {
            let (data, tx) = (params.clone(), tx.clone());
            thread::spawn(move || WinIOCP::safe_worker(data, tx)).join();
        }
    }

    fn InitializeListen(&mut self) -> Result<i32, i32> {

        let sockfd = unsafe { WSASocketW(AF_INET, SOCK_STREAM, 0, ptr::null_mut(), 0, WSA_FLAG_OVERLAPPED) };
        if sockfd == INVALID_SOCKET {
            println!("Invalid socket! {:?}", sockfd);
            return Err(-1);
        }

        // 将Listen Socket绑定至完成端口中
        let mut sock_context = PerSocketContext::new();
        let sock_context_raw = &mut sock_context as *mut _ as *mut raw::c_void;
        if ptr::null_mut() == unsafe {
            kernel32::CreateIoCompletionPort(
                sockfd as u64,
                self.iocp,
                sock_context_raw as u64,
                0)
            }
        {
            println!("Bind socket to IOCP failed!");
            return Err(-1);
        }

        let (addr, addr_len) = WinIOCP::get_addr_len();
        println!("Socket addr: {:?} len: {:?}", addr, addr_len);

        if 0 > unsafe { bind(sockfd, &addr as *const SOCKADDR, addr_len) } {
            println!("Bind socket failed!");
            return Err(-1);
        }

        if 0 > unsafe { listen(sockfd, SOMAXCONN) } {
            println!("Socket listen failed!");
            return Err(-1);
        }
        println!("Listenning...");

        Ok(0)
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

        if ret < 0 {
            unsafe { WSACleanup() };
            println!("WSAStartup failed! {:?}", ret);
            return Err(ret);
        }

        println!("WinSock version: {:?}, {:?}", LOBYTE(wsa_data.wVersion), HIBYTE(wsa_data.wVersion));

        if LOBYTE(wsa_data.wVersion) != 2 && HIBYTE(wsa_data.wVersion) != 2 {
            unsafe { WSACleanup() };
            println!("WinSock version Invalid!");
            return Err(-1);
        }

        Ok(ret)
    }

    fn get_sys_info() -> SYSTEM_INFO {
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

        unsafe { kernel32::GetSystemInfo(&mut info) };

        println!("System processors: {:?}", info.dwNumberOfProcessors);

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

    fn safe_worker(param: Arc<Mutex<WinIOCP>>, tx: mpsc::Sender<()>) {
        let mut data = param.lock().unwrap();
        let this = data.deref();
        println!("Work thread: {:?}", this.iocp);

        let mut count: u32 = 0;
        let mut socket_fd = INVALID_SOCKET;
        let mut over_lapped = ptr::null_mut::<LPOVERLAPPED>();
        let mut sock_context = PerSocketContext::new();
        let mut sock_context_raw = &mut sock_context as *mut _ as *mut u64;

        let status = unsafe {
            kernel32::GetQueuedCompletionStatus(
                this.iocp,
                &mut count,
                sock_context_raw,
                over_lapped,
                INFINITE
            )
        };
        println!("GetQueuedCompletionStatus:{:?}", status);

        tx.send(()).unwrap();
    }

}
