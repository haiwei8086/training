use std;
use std::{mem, ptr};
use winapi;
use kernel32;
use ws2_32;
use super::{consts};


extern "system" {
    fn console_handler(ctrlType: u32) -> i32 {
        println!("Ctrl Type: {}", ctrlType);

        return 1;
    }
}


pub struct Process {}

impl Process {

    pub fn new() {
        let pid = unsafe { kernel32::GetCurrentProcessId() };
        println!("Process new, PID: {}", pid);

        Process::info();
        Process::os_init();
        Process::set_console_handler();
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

    pub fn daemon() {
        println!("Run in daemon --------------------------------", );

        match unsafe { kernel32::FreeConsole() } {
                0 => println!("Free console failed!"),
                _ => println!("successed no print, Free console"),
        }
    }

    pub fn set_console_handler() {
        match unsafe { kernel32::SetConsoleCtrlHandler(Some(Process::console_handler), 1) } {
            0 => println!("Set Console Ctrl Handler failed!"),
            _ => println!("Set Console Ctrl Handler done"),
        }
    }

}
