#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ptr, env, path, process};
use std::collections::{BTreeMap};

use winapi::um::winnt::{HANDLE};

use winapi::um::handleapi::CloseHandle;



pub struct SignalEvent {
    pub name: &'static str,
    pub path_format: &'static str,
    pub path: String,
    pub handle: HANDLE,
}


pub struct Context {
    pub pid: u32,

    pub cpus: u8,
    pub memery_page_size: u32,
    pub allocation_granularity: u32,
    pub pid_file_name: &'static str,
    pub pid_path: path::PathBuf,

    pub arguments: BTreeMap<String, String>,

    pub events: Vec<SignalEvent>,

    pub stop: bool,
}



impl<'a> Context {

    pub fn new() -> Self {
        
        let mut ctx = Context {
            pid: process::id(),
            cpus: 2,
            memery_page_size: 4096,
            allocation_granularity: 65536,

            pid_file_name: "pid.lock",
            pid_path: env::current_dir().unwrap(),

            arguments: BTreeMap::new(),
            events: Vec::new(), 

            stop: false,
        };
        

        ctx.init_args();
        ctx.init_events();


        ctx
    }


    pub fn init_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        let len = args.len();
        
        
        let mut key: &String;
        let mut val: &String = &String::new();
        let mut i = 1;
        
        while i < len {
            key = &args[i];
            i += 1;

            if i < len {
                val = &args[i];
            }

            self.arguments.insert(key.to_owned(), val.to_owned());

            i += 1;
        }
    }


    pub fn init_events(&mut self) {
        self.events.push(SignalEvent {
            name: "stop",
            path_format: "Global\\stop_event_{}",
            path: format!("Global\\stop_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "quit",
            path_format: "Global\\quit_event_{}",
            path: format!("Global\\quit_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reopen",
            path_format: "Global\\reopen_event_{}",
            path: format!("Global\\reopen_event_{}", self.pid),
            handle: ptr::null_mut(),
        });
        self.events.push(SignalEvent {
            name: "reload",
            path_format: "Global\\reload_event_{}",
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