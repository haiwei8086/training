use std;
use std::{mem, ptr, io};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi;
use winapi::c_void;
use winapi::processthreadsapi::{PROCESS_INFORMATION, STARTUPINFOW};
use winapi::shlobj::INVALID_HANDLE_VALUE;
use winapi::winbase::CREATE_NO_WINDOW;
use winapi::winsock2::INVALID_SOCKET;
use winapi::winsock2::SOCKET;
use winapi::minwindef::LPVOID;
use winapi::minwindef::LPDWORD;
use winapi::minwinbase::OVERLAPPED;
use winapi::winsock2::LPWSAOVERLAPPED;
use winapi::winsock2::LPWSAOVERLAPPED_COMPLETION_ROUTINE;
use winapi::ws2def::SIO_GET_EXTENSION_FUNCTION_POINTER;
use winapi::guiddef::GUID;
use winapi::winnt::PVOID;
use kernel32;
use ws2_32;

use super::{consts, AddressFamily, SockType, console_handler};

extern "system" {

    pub fn WSAIoctl(
        s: SOCKET,
        dwIoControlCode: u32,
        lpvInBuffer: LPVOID,
        cbInBuffer: u32,
        lpvOutBuffer: LPVOID,
        cbOutBuffer: u32,
        lpcbBytesReturned: LPDWORD,
        lpOverlapped: LPVOID,
        lpCompletionRoutine: LPVOID,
    ) -> i32;

    pub fn LPFN_ACCEPTEX(
        sListenSocket: SOCKET,
        sAcceptSocket: SOCKET,
        lpOutputBuffer: PVOID,
        dwReceiveDataLength: u32,
        dwLocalAddressLength: u32,
        dwRemoteAddressLength: u32,
        lpdwBytesReceived: *mut u32,
        lpOverlapped: *mut OVERLAPPED,
    ) -> bool;
}


static mut is_worker: bool = false;

pub struct Process {
    is_worker: bool,
}

impl Process {
    pub fn new() {
        let pid = unsafe { kernel32::GetCurrentProcessId() };
        println!("Process new, PID: {}", pid);

        Process::info();
        Process::os_init();
        Process::set_console_handler();
        Process::create_signal_events();
        // Process::create_process();

        // loop input

        let mut input = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.as_str() {
                "q" => break,
                _ => println!("You input: {}", input),
            }
        }

    }

    pub fn info() {
        println!("Process infos ----------------------------");
        println!("Current directory: {}",
                 std::env::current_dir().unwrap().display());
        println!("Current exe: {:?}", std::env::current_exe());

        for argument in std::env::args() {
            println!("Arguments item: {}", argument);
            if argument.trim() == "--worker" {
                unsafe {
                    is_worker = true;
                }
                println!("This is a worker thread");
            }
        }

        println!("OS Version: {:?}", unsafe { kernel32::GetVersion() });

        let mut sys_info: winapi::sysinfoapi::SYSTEM_INFO;
        unsafe {
            sys_info = mem::zeroed();
            kernel32::GetSystemInfo(&mut sys_info);
        }
        println!("System info: {:?}", sys_info);
    }

    pub fn os_init() {
        println!("OS Init -------------------------------");

        let mut wsa_data: winapi::winsock2::WSADATA;
        let ret = unsafe {
            wsa_data = mem::zeroed();
            ws2_32::WSAStartup(0x202, &mut wsa_data)
        };

        match ret {
            -1 => println!("Init Winsock failed: {}", ret),
            _ => println!("Init Winsock successed: {}", ret),
        }

        let socket_fd = unsafe {
            ws2_32::WSASocketW(
                AddressFamily::Inet as i32,
                SockType::Stream as i32,
                consts::IPPROTO_IP,
                ptr::null_mut(),
                0,
                consts::WSA_FLAG_OVERLAPPED)
        };

        match socket_fd {
            INVALID_SOCKET => println!("Create socket failed!"),
            _ => println!("Socket created"),
        }


        let mut WSAID_ACCEPTEX = GUID {
            Data1: 0xb5367df1,
            Data2: 0xcbac,
            Data3: 0x11cf,
            Data4: [0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92],
        };

        let mut byte_len: u32 = 0;
        let mut accept_fn = LPFN_ACCEPTEX;
        let mut over_lapped: OVERLAPPED;

        let ret = unsafe {
            over_lapped = mem::zeroed::<OVERLAPPED>();

            println!("WSAIoctl socket_fd: {:?}", socket_fd);
            println!("WSAIoctl dwIoControlCode: {:?}", SIO_GET_EXTENSION_FUNCTION_POINTER);
            println!("WSAIoctl lpvInBuffer: {:?}", &mut WSAID_ACCEPTEX as *mut _ as *mut c_void);
            println!("WSAIoctl cbInBuffer: {:?}", mem::size_of::<GUID>() as u32);
            println!("WSAIoctl lpvOutBuffer: {:?}", &mut accept_fn as *mut _ as *mut c_void);
            println!("WSAIoctl cbOutBuffer: {:?}", mem::size_of::<LPVOID>() as u32);
            println!("WSAIoctl lpcbBytesReturned: {:?}", &mut byte_len);
            println!("WSAIoctl lpOverlapped: {:?}", &mut over_lapped);
            //println!("WSAIoctl lpCompletionRoutine: {:?}", ptr::null());

            WSAIoctl(
                socket_fd,
                SIO_GET_EXTENSION_FUNCTION_POINTER,
                &mut WSAID_ACCEPTEX as *mut _ as *mut c_void,
                mem::size_of::<GUID>() as u32,
                &mut accept_fn as *mut _ as *mut c_void,
                mem::size_of::<LPVOID>() as u32,
                &mut byte_len,
                ptr::null_mut(),
                ptr::null_mut()
            )
        };



        match ret {
            -1 => println!("Get AcceptEx fn failed"),
            _ => println!("Get AcceptEx fn: {:?}", ret),
        }


        match unsafe { ws2_32::closesocket(socket_fd) } {
            -1 => println!("Close socket failed!"),
            _ => println!("Socket closed"),

        }
        unsafe { ws2_32::WSACleanup() };


        // TODO: get acceptx fn pointer
        //
    }

    pub fn daemon() {
        println!("Run in daemon --------------------------------", );

        match unsafe { kernel32::FreeConsole() } {
            0 => println!("Free console failed!"),
            _ => println!("successed no print, Free console"),
        }
    }

    pub fn set_console_handler() {
        match unsafe { kernel32::SetConsoleCtrlHandler(Some(console_handler), 1) } {
            0 => println!("Set Console Ctrl Handler failed!"),
            _ => println!("Set Console Ctrl Handler done"),
        }
    }

    pub fn to_wchar(str: &str) -> *const u16 {
        let v: Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();

        v.as_ptr()
    }

    pub fn create_signal_events() {

        println!("Create events ---------------------------------");

        let stop_event_name = Process::to_wchar("stop_event");
        let stop_event = unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, stop_event_name) };
        if stop_event.is_null() {
            println!("Create stop_event failed!");
        } else {
            println!("Create stop_event");
        }

        let quit_event_name = Process::to_wchar("quit_event");
        let quit_event = unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, quit_event_name) };
        if quit_event.is_null() {
            println!("Create quit_event failed!");
        } else {
            println!("Create quit_event");
        }

        let reopen_event_name = Process::to_wchar("quit_event");
        let reopen_event =
            unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, reopen_event_name) };
        if reopen_event.is_null() {
            println!("Create reopen_event failed!");
        } else {
            println!("Create reopen_event");
        }

        let reload_event_name = Process::to_wchar("quit_event");
        let reload_event =
            unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, reload_event_name) };
        if reload_event.is_null() {
            println!("Create reload_event failed!");
        } else {
            println!("Create reload_event");
        }

        unsafe {
            kernel32::CloseHandle(stop_event);
            kernel32::CloseHandle(quit_event);
            kernel32::CloseHandle(reopen_event);
            kernel32::CloseHandle(reload_event);
        }
    }


    pub fn zeroed_process_information() -> PROCESS_INFORMATION {
        PROCESS_INFORMATION {
            hProcess: ptr::null_mut(),
            hThread: ptr::null_mut(),
            dwProcessId: 0,
            dwThreadId: 0,
        }
    }

    pub fn zeroed_startupinfo() -> STARTUPINFOW {
        STARTUPINFOW {
            cb: 0,
            lpReserved: ptr::null_mut(),
            lpDesktop: ptr::null_mut(),
            lpTitle: ptr::null_mut(),
            dwX: 0,
            dwY: 0,
            dwXSize: 0,
            dwYSize: 0,
            dwXCountChars: 0,
            dwYCountChars: 0,
            dwFillAttribute: 0,
            dwFlags: 0,
            wShowWindow: 0,
            cbReserved2: 0,
            lpReserved2: ptr::null_mut(),
            hStdInput: INVALID_HANDLE_VALUE,
            hStdOutput: INVALID_HANDLE_VALUE,
            hStdError: INVALID_HANDLE_VALUE,
        }
    }

    pub fn create_process() {
        if unsafe { is_worker } {
            return;
        }

        println!("Create process ---------------------------------");
        println!("Program path: {:?}", std::env::current_exe().unwrap());

        let program = Process::to_wchar(std::env::current_exe().unwrap().to_str().unwrap());
        let args = Process::to_wchar(" --worker");

        for x in 0..2 {
            println!("Process: {:?}", x);
            let mut statu_info = Process::zeroed_startupinfo();
            let mut process_info = Process::zeroed_process_information();

            let ret = unsafe {
                kernel32::CreateProcessW(program as *const u16,
                                         args as *mut u16,
                                         ptr::null_mut(),
                                         ptr::null_mut(),
                                         0,
                                         CREATE_NO_WINDOW,
                                         ptr::null_mut(),
                                         ptr::null_mut(),
                                         &mut statu_info,
                                         &mut process_info)
            };

            match ret {
                0 => {
                    println!("Create process failed!");
                    println!("Status info: {:?}", &statu_info);
                    println!("Process info: {:?}", &process_info);
                }
                _ => {
                    unsafe {
                        // kernel32::CloseHandle(process_info.hProcess);
                        kernel32::CloseHandle(process_info.hThread);
                    }
                }
            }
        }
    }
}
