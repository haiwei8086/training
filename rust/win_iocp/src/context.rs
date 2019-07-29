#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr, process};
use winapi::um::winnt::{HANDLE};

use winapi::um::handleapi::CloseHandle;



pub struct SignalEvent {
    pub name: &'static str,
    pub path: String,
    pub handle: HANDLE,
}


pub struct Context {
    pub pid: u32,
    pub events: Vec<SignalEvent>,

}



impl Context {

    pub fn new() -> Self {
        let mut ctx = Context {
            pid: process::id(),
            events: Vec::new(), 
        };
        

        ctx.events.push(SignalEvent {
            name: "stop",
            path: format!("Global\\stop_event_{}", ctx.pid),
            handle: ptr::null_mut(),
        });
        ctx.events.push(SignalEvent {
            name: "quit",
            path: format!("Global\\quit_event_{}", ctx.pid),
            handle: ptr::null_mut(),
        });
        ctx.events.push(SignalEvent {
            name: "reopen",
            path: format!("Global\\reopen_event_{}", ctx.pid),
            handle: ptr::null_mut(),
        });
        ctx.events.push(SignalEvent {
            name: "reload",
            path: format!("Global\\reload_event_{}", ctx.pid),
            handle: ptr::null_mut(),
        });


        ctx
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