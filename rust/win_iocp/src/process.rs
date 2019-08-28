#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr, thread, time};
use std::sync::{Arc, Weak, Mutex};
use std::io::Error;
use std::ops::Not;
use std::ffi::{OsString, OsStr};
use std::os::windows::prelude::*;

use winapi::um::winbase::{INFINITE, WAIT_OBJECT_0, WAIT_FAILED};
use winapi::um::wincon::{
    CTRL_C_EVENT, 
    CTRL_BREAK_EVENT, 
    CTRL_CLOSE_EVENT, 
    CTRL_LOGOFF_EVENT, 
    CTRL_SHUTDOWN_EVENT
};
use winapi::um::winnt::{LPCWSTR, HANDLE, EVENT_MODIFY_STATE};
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::WAIT_TIMEOUT;

use winapi::um::consoleapi::SetConsoleCtrlHandler;
use winapi::um::wincon::FreeConsole;
use winapi::um::synchapi::{    
    CreateMutexW as CreateMutex,
    ReleaseMutex,

    WaitForMultipleObjects,
    WaitForSingleObject,    
    
    CreateEventW as CreateEvent, 
    OpenEventW as OpenEvent,
    SetEvent,
    ResetEvent,
};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcessId;

use super::os::{self, to_wchar};
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
        let mut lp_name: Vec<u16> = to_wchar(event.path.as_str());
        let ev = unsafe { CreateEvent(ptr::null_mut(), 1, 0, lp_name.as_mut_ptr()) };

        if ev.is_null() {
            println!("CreateEvent({}) failed!", event.path);
        } else {
            println!("CreateEvent {}", event.path);

            event.handle = ev;
        }
    }
}


pub fn signal_process(ctx: &Context, signal: &str) {

    if let Some(_event) = ctx.events.iter().find(|&e| e.name == signal) {
        println!("[signal_process] Find event: {}", _event.name);

        let pid = os::read_pid(&ctx).unwrap();

        let event_path = _event.path_format.replace("{}", &pid.to_string());
        let mut lp_name: Vec<u16> = to_wchar(event_path.as_str());

        let ev = unsafe { OpenEvent(EVENT_MODIFY_STATE, 0, lp_name.as_mut_ptr()) };
        if ev.is_null() {
            println!("[signal_process] OpenEvent({}) failed! error: {:?}", event_path, Error::last_os_error());
            return;
        }
    
        println!("[signal_process] OpenEvent {}", event_path);


        let ret = unsafe { SetEvent(ev) };
        if ret == 0 {
            println!("[signal_process] SetEvent({:?}) failed!", ev);
            return;
        }
        println!("[signal_process] SetEvent {}", event_path);


        unsafe { CloseHandle(ev) };
    }
}


pub fn master_process_cycle(ctx: &mut Context) {
    let process_type_key: &str = "pt";

    let process_type: String = os::get_environment(process_type_key);

    if process_type == "work" {
        println!("Process type is work.");
        // start worker
    }


    println!("[master_process_cycle]");

    os::set_environment(process_type_key, "master");


    create_signal_events(&mut ctx.events);
    for event in &mut ctx.events {
        println!("Signal event: {:?} {}", event.handle, event.path);

        /*
        let ret = unsafe { CloseHandle(event.handle) };
        println!("CloseHandle ret: {:?}", ret);


        event.handle = ptr::null_mut();
        */
    }

    let mut master_process_event_name: Vec<u16> = to_wchar(format!("master_event_{}", ctx.pid).as_str());
    let master_process_event = unsafe { CreateEvent(ptr::null_mut(), 1, 0, master_process_event_name.as_mut_ptr()) };
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


    let mut failed_count = 0;
    let mut ev: DWORD;

    loop {
        println!("master_process_cycle loop...");

        if failed_count > 5 {
            break;
        }

        ev = unsafe {
            WaitForMultipleObjects(events.len() as DWORD, events.as_ptr(), 0, INFINITE)
        };
    

        println!("master WaitForMultipleObjects: {}", ev);


        if ev == WAIT_OBJECT_0 {
            println!("exiting by stop_event.");

            break master_process_exit()
        } 

        if ev == WAIT_OBJECT_0 + 1 {
            println!("shutting down by quit_event.");

            break master_process_exit()
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

            break master_process_exit()
        } 

        if ev == WAIT_TIMEOUT {
            println!("timeout");

            break master_process_exit()
        } 


        if ev == WAIT_FAILED {
            println!("master WaitForMultipleObjects failed. reason: {:?}", Error::last_os_error());

            failed_count += 1;

            continue;
            //break master_process_exit()
        }
    }
}


pub fn single_process_cycle(ctx: &mut Context) {
    println!("[single_process_cycle]");


    create_signal_events(&mut ctx.events);
    for event in &mut ctx.events {
        println!("[single_process_cycle] Signal event: {:?} {}", event.handle, event.path);
    }

    let arc_ctx = Arc::new(&*ctx);
    let worker = worker_thread(Arc::downgrade(&arc_ctx));


    if let Some(_event) = ctx.events.iter().find(|&e| e.name == "stop") {
        println!("[signal_process] WaitForSingleObject {}", _event.name);

        unsafe {
            WaitForSingleObject(_event.handle, INFINITE)
        };

        ctx.stop = true;

        worker.join().unwrap();
    }
}


pub fn master_process_exit() {

    println!("master_process_exit");

}


fn worker_thread(ctx: Weak<&Context>) -> thread::JoinHandle<()> {

    let worker = thread::spawn(move || {
        println!("Worker thread.");

        let wait_millis = time::Duration::from_millis(1 * 1000);

        while *ctx.stop {
            println!("in working...");

            thread::sleep(wait_millis);
        }
    });


    worker
}