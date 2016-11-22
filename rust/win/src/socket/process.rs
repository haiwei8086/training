use std;
use std::{mem, ptr};
use winapi;
use kernel32;
use ws2_32;
use super::{consts};

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

        unsafe { ws2_32::WSACleanup() };


        /*
            TODO: get acceptx fn pointer
        */
    }

}
