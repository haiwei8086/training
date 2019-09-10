#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr, env, mem, path, process};
use std::collections::{BTreeMap};


use winapi::um::winnt::HANDLE;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::shared::ws2def::{WSABUF};
use winapi::um::winsock2::{u_long, SOCKET, INVALID_SOCKET};

use winapi::um::handleapi::CloseHandle;
use winapi::um::winsock2::{closesocket, shutdown};


pub const MAX_THREADS: u32 = 2;
pub const MAX_BUFFER_LEN: u32 = 4096;


pub struct SignalEvent {
    pub name: &'static str,
    pub path_format: &'static str,
    pub path: String,
    pub handle: HANDLE,
}


pub struct Context {
    pub pid: u32,

    pub cpus: u8,
    pub memery_page_size: u32,
    pub allocation_granularity: u32,
    pub pid_file_name: &'static str,
    pub pid_path: path::PathBuf,

    pub arguments: BTreeMap<String, String>,

    pub events: Vec<SignalEvent>,

    pub stop: bool,
}


impl<'a> Context {

    pub fn new() -> Self {
        
        let mut ctx = Context {
            pid: process::id(),
            cpus: 2,
            memery_page_size: 4096,
            allocation_granularity: 65536,

            pid_file_name: "pid.lock",
            pid_path: env::current_dir().unwrap(),

            arguments: BTreeMap::new(),
            events: Vec::new(), 

            stop: false,
        };
        

        ctx.init_args();
        ctx.init_events();


        ctx
    }


    pub fn init_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        let len = args.len();
        
        
        let mut key: &String;
        let mut val: &String = &String::new();
        let mut i = 1;
        
        while i < len {
            key = &args[i];
            i += 1;

            if i < len {
                val = &args[i];
            }

            self.arguments.insert(key.to_owned(), val.to_owned());

            i += 1;
        }
    }


    pub fn init_events(&mut self) {
        self.events.push(SignalEvent {
            name: "stop",
            path_format: "Global\\stop_event_{}",
            path: format!("Global\\stop_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "quit",
            path_format: "Global\\quit_event_{}",
            path: format!("Global\\quit_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reopen",
            path_format: "Global\\reopen_event_{}",
            path: format!("Global\\reopen_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reload",
            path_format: "Global\\reload_event_{}",
            path: format!("Global\\reload_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
    }
}


impl Drop for Context {

    fn drop(&mut self) {
        println!("Drop context");

        for event in &self.events {
            if !event.handle.is_null() {
                println!("CloseHandle: {:?} {}", event.handle, event.path);
                
                unsafe { CloseHandle(event.handle) };
            }
        }

    }
    
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
