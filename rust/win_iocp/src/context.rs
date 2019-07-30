#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr, env, process};
use std::collections::{BTreeMap};

use winapi::um::winnt::{HANDLE};

use winapi::um::handleapi::CloseHandle;



pub struct SignalEvent {
    pub name: &'static str,
    pub path: String,
    pub handle: HANDLE,
}


pub struct Context {
    pub pid: u32,

    pub cpus: u8,
    pub memery_page_size: u32,
    pub allocation_granularity: u32,

    pub argument_map: BTreeMap<String, String>,

    pub events: Vec<SignalEvent>,

}



impl<'a> Context {

    pub fn new() -> Self {
        let mut ctx = Context {
            pid: process::id(),
            cpus: 2,
            memery_page_size: 4096,
            allocation_granularity: 65536,

            argument_map: BTreeMap::new(),
            events: Vec::new(), 
        };
        

        ctx.init_args();
        ctx.init_events();


        ctx
    }


    pub fn init_args(&mut self) {
        let args = env::args();
        
        let mut i = 0;
        let mut key_list: Vec<String> = Vec::new();
        let mut val_list: Vec<String> = Vec::new();

        for arg in args {
            if i > 0 {
                if i%2 == 0 {
                    val_list.push(arg);
                } else {
                    key_list.push(arg);
                }
            }

            i += 1;
        }

        i = 0;
        let v_len = val_list.len();

        for k in key_list {
            if i < v_len {
                self.argument_map.insert(k, val_list[i].to_owned());
            } else {
                self.argument_map.insert(k, "".to_string());
            }

            i += 1;
        }
    }


    pub fn init_events(&mut self) {
        self.events.push(SignalEvent {
            name: "stop",
            path: format!("Global\\stop_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "quit",
            path: format!("Global\\quit_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reopen",
            path: format!("Global\\reopen_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reload",
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