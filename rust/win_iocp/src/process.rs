#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr};

use winapi::um::winbase::{INFINITE, WAIT_OBJECT_0, WAIT_FAILED};
use winapi::um::wincon::{
    CTRL_C_EVENT, 
    CTRL_BREAK_EVENT, 
    CTRL_CLOSE_EVENT, 
    CTRL_LOGOFF_EVENT, 
    CTRL_SHUTDOWN_EVENT
};
use winapi::um::winnt::{LPCWSTR, HANDLE};
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::WAIT_TIMEOUT;


use winapi::um::consoleapi::SetConsoleCtrlHandler;
use winapi::um::wincon::FreeConsole;
use winapi::um::synchapi::{
    CreateEventW as CreateEvent, 
    CreateMutexW as CreateMutex,
    WaitForMultipleObjects,
    WaitForSingleObject,
    ReleaseMutex,
    ResetEvent,
};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcessId;


use super::os::{convert_to_wchar};
use super::context::{SignalEvent, Context};



pub fn set_console_handler() {
    let ret = unsafe { SetConsoleCtrlHandler(Some(console_handler), 1) }; 

    match ret {
        0 => println!("Set Console Ctrl Handler failed."),
        _ => println!("Set Console Ctrl Handler"),
    };
}

// for SetConsoleCtrlHandler
pub unsafe extern "system" fn console_handler(ctrl_type: u32) -> i32 {

    match ctrl_type {
        CTRL_C_EVENT => println!("Ctrl-C pressed, exiting"),
        CTRL_BREAK_EVENT => println!("Ctrl-Break pressed, exiting"),
        CTRL_CLOSE_EVENT => println!("console closing, exiting"),
        CTRL_LOGOFF_EVENT => println!("user logs off, exiting"),
        CTRL_SHUTDOWN_EVENT => println!("Ctrl-shutdown pressed, exiting"),

        _ => println!("Console Ctrl Handler: {}", ctrl_type),
    }

    return 1;
}

pub fn free_console() {
    match unsafe { FreeConsole() } {
        0 => println!("Free console failed!"),
        _ => println!("Free console."),
    };
}


pub fn create_signal_events(events: &mut Vec<SignalEvent>) {

    for event in events {
        let ev = unsafe { CreateEvent(ptr::null_mut(), 1, 0, convert_to_wchar(event.path.as_str())) };

        if ev.is_null() {
            println!("CreateEvent({}) failed!", event.path);
        } else {
            println!("CreateEvent {}", event.path);

            event.handle = ev;
        }
    }
}


pub fn master_process_cycle(ctx: &mut Context) {
    println!("master_process_cycle");


    create_signal_events(&mut ctx.events);
    for event in &mut ctx.events {
        println!("Signal event: {:?} {}", event.handle, event.path);

        let ret = unsafe { CloseHandle(event.handle) };
        println!("CloseHandle ret: {:?}", ret);

        event.handle = ptr::null_mut();
    }


    let master_process_event_name = convert_to_wchar((format!("master_event_{}", ctx.pid)).as_str());
    let master_process_event = unsafe { CreateEvent(ptr::null_mut(), 1, 0, master_process_event_name) };
    if master_process_event.is_null() {
        println!("Create master_process_event failed!");
    } else {
        println!("Create master_process_event");
    }
    // TODO
    unsafe {
        CloseHandle(master_process_event);
    }


    let mut events: Vec<HANDLE> = Vec::with_capacity(ctx.events.len());
    for ev in &ctx.events {
        events.push(ev.handle);
    }


    let mut ev: DWORD;

    loop {
        println!("master_process_cycle loop...");

        ev = unsafe {
            WaitForMultipleObjects(events.len() as DWORD, events.as_ptr(), 0, INFINITE)
        };
    

        println!("master WaitForMultipleObjects: {}", ev);


        if ev == WAIT_OBJECT_0 {
            println!("exiting by stop_event.");

            master_process_exit();
            break;
        } 

        if ev == WAIT_OBJECT_0 + 1 {
            println!("shutting down by quit_event.");

            master_process_exit();
            break;
        } 

        if ev == WAIT_OBJECT_0 + 2 {
            println!("reopening by reopen_event.");

            if 0 == unsafe { ResetEvent(events[2]) } {
                println!("ResetEvent reopen_event failed.");
            }


            continue;
        } 

        if ev == WAIT_OBJECT_0 + 3 {
            println!("reconfiguring by reload_event.");

            if 0 == unsafe { ResetEvent(events[3]) } {
                println!("ResetEvent reload_event failed.");
            }

            continue;
        } 


        if ev > WAIT_OBJECT_0 + 3 && ev < WAIT_OBJECT_0 + events.len() as u32 {
            println!("reap worker");

            master_process_exit();
            break;
        } 

        if ev == WAIT_TIMEOUT {
            println!("timeout");

            master_process_exit();
            break;
        } 


        if ev == WAIT_FAILED {
            println!("master WaitForMultipleObjects failed.");

            // continue;
            break master_process_exit()
        }
    }
}


pub fn master_process_exit() {

    println!("master_process_exit");

}