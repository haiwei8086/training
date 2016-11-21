use std;
use std::{mem, ptr};
use winapi;
use kernel32;
use ws2_32;
use super::{consts, ffi, AddressFamily, SockType};

pub struct Process {}

impl Process {

    pub fn new() {
        let pid = unsafe { kernel32::GetCurrentProcessId() };
        println!("Process new, PID: {}", pid);

        Process::info();
        Process::os_init();
    }

    pub fn info() {
        println!("Process infos ----------------------------");
        println!("Current directory: {}", std::env::current_dir().unwrap().display());
        println!("Current exe: {:?}", std::env::current_exe());

        for argument in std::env::args() {
            println!("Arguments item: {}", argument);
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
        let mut wsa_data: winapi::winsock2::WSADATA;

        let result = unsafe {
            wsa_data = mem::zeroed();
            ws2_32::WSAStartup(0x202, &mut wsa_data)
        };

        match result {
            -1 => println!("Init Winsock failed: {}", result),
            _ => println!("Init Winsock successed: {}", result),
        }

        let socketfd = unsafe {
            ws2_32::WSASocketW(
                AddressFamily::Inet as i32,
                SockType::Stream as i32,
                consts::IPPROTO_IP,
                ptr::null_mut(),
                0,
                consts::WSA_FLAG_OVERLAPPED)
        };

        match socketfd {
            winapi::INVALID_SOCKET => println!("WSASocketW INVALID_SOCKET: {:?}", socketfd),
            _ => println!("WSASocketW successed: {:?}", socketfd),
        }

        let ret: i32;
        let mut WSAID_ACCEPTEX = winapi::guiddef::GUID {
            Data1: 0xb5367df1,
            Data2: 0xcbac,
            Data3: 0x11cf,
            Data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
        };


        let mut fn_acceptex: system::LPFN_ACCEPTEX = ptr::null();

        let mut bytes: winapi::DWORD = 0;

        println!("Size: {:?}", fn_acceptex);

        let complete: Option<unsafe  extern "system" fn(
            dwError: winapi::DWORD,
            cbTransferred: winapi::DWORD,
            lpOverlapped: winapi::LPWSAOVERLAPPED,
            dwFlags: winapi::DWORD)> = unsafe { mem::zeroed() };

/*
        ret = unsafe {
            ws2_32::WSAIoctl(
                socketfd,
                winapi::SIO_GET_EXTENSION_FUNCTION_POINTER,
                &mut WSAID_ACCEPTEX as *mut _ as *mut std::os::raw::c_void,
                mem::size_of::<winapi::guiddef::GUID>() as u32,
                &mut fn_acceptex as *mut _ as *mut std::os::raw::c_void,
                mem::size_of::<*mut fn_acceptex>() as u32,
                &mut bytes,
                ptr::null_mut(),
                complete
            )
        };
        println!("WSAIoctl: WSAID_ACCEPTEX: {:?}", ret);
*/
    }

}
