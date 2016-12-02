use std;
use std::{mem, ptr, io};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi;
use kernel32;
use ws2_32;
use super::{console_handler};



pub struct Process {}

impl Process {

    pub fn new() {
        let pid = unsafe { kernel32::GetCurrentProcessId() };
        println!("Process new, PID: {}", pid);

        Process::info();
        Process::os_init();
        Process::set_console_handler();
        Process::create_signal_events();
        Process::create_process();

        // loop input
        /*
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
        */
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
        match unsafe { kernel32::SetConsoleCtrlHandler(Some(console_handler), 1) } {
            0 => println!("Set Console Ctrl Handler failed!"),
            _ => println!("Set Console Ctrl Handler done"),
        }
    }

    pub fn to_wchar(str : &str) -> *const u16 {
        let v : Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();

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
        let reopen_event = unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, reopen_event_name) };
        if reopen_event.is_null() {
            println!("Create reopen_event failed!");
        } else {
            println!("Create reopen_event");
        }

        let reload_event_name = Process::to_wchar("quit_event");
        let reload_event = unsafe { kernel32::CreateEventW(ptr::null_mut(), 1, 0, reload_event_name) };
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

    pub fn create_process() {
        println!("Create process ---------------------------------");

        for x in 0..2 {
            println!("Process: {:?}", x);


            let mut program = std::env::current_exe().unwrap();
            println!("Program path: {:?}", program);

            let mut statu_info: winapi::processthreadsapi::STARTUPINFOW;
            let mut process_info: winapi::processthreadsapi::PROCESS_INFORMATION;

            unsafe {
                statu_info = mem::zeroed::<winapi::processthreadsapi::STARTUPINFOW>();
                process_info = mem::zeroed::<winapi::processthreadsapi::PROCESS_INFORMATION>();

                let mut path = Process::to_wchar(program.to_str().unwrap()) as *mut u16;

                match kernel32::CreateProcessW(
                    ptr::null_mut(),
                    path,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    0,
                    winapi::winbase::CREATE_NO_WINDOW,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &mut statu_info,
                    &mut process_info) {
                        0 => {
                            println!("Create process failed!");
                            println!("Status info: {:?}", &statu_info);
                            println!("Process info: {:?}", &process_info);
                        },
                        _ => {
                            kernel32::CloseHandle(process_info.hProcess);
                            kernel32::CloseHandle(process_info.hThread);
                        },
                }
            }
        }
    }

}
