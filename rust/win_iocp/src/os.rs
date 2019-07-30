use std::{env, path, fs, mem, ptr};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::io::prelude::*;
use std::str::FromStr;


use winapi::ctypes::c_void;
use winapi::um::sysinfoapi::SYSTEM_INFO;
use winapi::um::winbase::MEMORYSTATUS;
use winapi::um::winsock2::WSADATA;
use winapi::um::winsock2::SOCKET;
use winapi::um::winsock2::INVALID_SOCKET;
use winapi::shared::ws2def::SIO_GET_EXTENSION_FUNCTION_POINTER;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::LPVOID;

use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::winbase::GlobalMemoryStatus;
use winapi::um::sysinfoapi::GetVersion;
use winapi::um::winsock2::WSAStartup;
use winapi::um::winsock2::WSACleanup;
use winapi::um::winsock2::WSASocketW;
use winapi::um::winsock2::closesocket;
use winapi::um::winsock2::WSAIoctl;
use winapi::um::processthreadsapi::GetCurrentProcessId;



use super::consts;
use super::ffi;
use super::context::Context;



pub fn init(ctx: &mut Context) {
    system_info(ctx);

    init_winsock();
}


fn system_info(ctx: &mut Context) {
    println!("-------------------------------------");
    println!("OS info:");

    println!("PID: {}", unsafe{ GetCurrentProcessId() });
    println!("Path: {}",  env::current_dir().unwrap().display());
    println!("Exe: {:?}", env::current_exe().unwrap());


    for argument in env::args() {
        println!("Argument: {}", argument);

        /*
        if argument.trim() == "--worker" {
            unsafe { is_worker = true; }
            println!("This is a worker thread");
        }
        */
    }


    println!("OS version: {:?}", unsafe { GetVersion() });


    let mut sys_info: SYSTEM_INFO;
    unsafe {
        sys_info = mem::zeroed();

        GetSystemInfo(&mut sys_info);
    }
    println!("System info:");
    println!("Page Size: {:?}", sys_info.dwPageSize);
    println!("Allocation Granularity: {:?}", sys_info.dwAllocationGranularity);
    println!("Minimum Application Address: {:?}", sys_info.lpMinimumApplicationAddress);
    println!("Maximum Application Address: {:?}", sys_info.lpMaximumApplicationAddress);
    println!("Active Processor Mask: {:?}", sys_info.dwActiveProcessorMask);
    println!("Number Of Processors: {:?}", sys_info.dwNumberOfProcessors);
    //println!("dwProcessorType: {:?}", sys_info.dwProcessorType);    
    println!("Processor Level: {:?}", sys_info.wProcessorLevel);
    println!("Processor Revision: {:?}", sys_info.wProcessorRevision);


    ctx.cpus = sys_info.dwNumberOfProcessors as u8;
    ctx.memery_page_size = sys_info.dwPageSize as u32;
    ctx.allocation_granularity = sys_info.dwAllocationGranularity as u32;


    let mut mem_status: MEMORYSTATUS;
    unsafe {
        mem_status = mem::zeroed();

        GlobalMemoryStatus(&mut mem_status);
    }
    println!("Blobal Memory Status:");
    println!("dwLength: {:?}", mem_status.dwLength);
    println!("dwMemoryLoad: {:?}", mem_status.dwMemoryLoad);
    println!("dwTotalPhys: {:?}", mem_status.dwTotalPhys);
    println!("dwAvailPhys: {:?}", mem_status.dwAvailPhys);
    println!("dwTotalPageFile: {:?}", mem_status.dwTotalPageFile);
    println!("dwAvailPageFile: {:?}", mem_status.dwAvailPageFile);
    println!("dwTotalVirtual: {:?}", mem_status.dwTotalVirtual);
    println!("dwAvailVirtual: {:?}", mem_status.dwAvailVirtual);
}


fn init_winsock() {
    println!("-------------------------------------");
    println!("Initialize Winsock.");


    let mut wsa_data: WSADATA;
    let ret = unsafe {
        wsa_data = mem::zeroed();

        WSAStartup(0x202, &mut wsa_data)
    };
    match ret {
        0 => println!("WSAStartup() successed."),
        _ => println!("WSAStartup() failed. code: {}", ret),
    }


    /*
     * get AcceptEx(), GetAcceptExSockAddrs(), TransmitFile(),
     * TransmitPackets(), ConnectEx(), and DisconnectEx() addresses
     */

    let socket_fd: SOCKET = unsafe {
        WSASocketW (
            consts::AF_INET,
            consts::SOCK_STREAM,
            consts::IPPROTO_IP,
            ptr::null_mut(),
            0,
            consts::WSA_FLAG_OVERLAPPED
        )
    };
    match socket_fd {
        INVALID_SOCKET => println!("Create socket failed!"),
        _ => println!("Socket created"),
    }


    let mut byte_len: u32 = 0;

    let mut ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_ACCEPTEX as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_AcceptEx as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get AcceptEx fn failed"),
        _ => println!("Get AcceptEx fn: {:?}", ret),
    }


    ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_GETACCEPTEXSOCKADDRS as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_GetAcceptExSockaddrs as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get GetAcceptExSockaddrs fn failed"),
        _ => println!("Get GetAcceptExSockaddrs fn: {:?}", ret),
    }
    

    ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_TRANSMITFILE as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_TransmitFile as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get TransmitFile fn failed"),
        _ => println!("Get TransmitFile fn: {:?}", ret),
    }


    ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_TRANSMITPACKETS as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_TransmitPackets as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get TransmitPackets fn failed"),
        _ => println!("Get TransmitPackets fn: {:?}", ret),
    }


    ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_CONNECTEX as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_ConnectEx as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get ConnectEx fn failed"),
        _ => println!("Get ConnectEx fn: {:?}", ret),
    }


    ret = unsafe {
        WSAIoctl (
            socket_fd,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut ffi::WSAID_DISCONNECTEX as *mut _ as *mut c_void,
            mem::size_of::<GUID>() as u32,
            &mut ffi::LPFN_DisconnectEx as *mut _ as *mut c_void,
            mem::size_of::<LPVOID>() as u32,
            &mut byte_len,
            ptr::null_mut(),
            None
        )
    };
    match ret {
        -1 => println!("Get DisconnectEx fn failed"),
        _ => println!("Get DisconnectEx fn: {:?}", ret),
    }



    match unsafe { closesocket(socket_fd) } {
        -1 => println!("closesocket() failed!"),
        _ => println!("Socket closed"),
    }
    unsafe { WSACleanup() };
}


pub fn to_wchar(str: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect()
}


pub fn save_pid(ctx: &Context) {

    let mut pid_dir = path::PathBuf::new();
    pid_dir.push(ctx.pid_path.as_path());
    pid_dir.push(ctx.pid_file_name);

    let mut file = fs::File::create(pid_dir.as_path()).unwrap();
    file.write_all((format!("{}", ctx.pid)).as_bytes()).unwrap();

    println!("Save pid file to {}", pid_dir.display());
}

pub fn read_pid(ctx: &Context) -> std::io::Result<u32> {
    let mut pid_dir = path::PathBuf::new();
    pid_dir.push(ctx.pid_path.as_path());
    pid_dir.push(ctx.pid_file_name);

    let mut file = fs::File::open(pid_dir.as_path())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    println!("read_pid read content: {}", contents);


    Ok(u32::from_str(&*contents).unwrap())
}