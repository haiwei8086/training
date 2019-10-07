use std::{ptr, io};
use std::os::windows::io::*;

use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;


pub struct IOCP(HANDLE);


impl IOCP {
    
    pub fn new() -> io::Result<Self> {

        let ret = unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 0) }
        if ret.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(IOCP(ret))
    }


    pub fn associate<T: AsRawSocket + ?Sized>(&self, fd: &T, key: usize) -> io::Result<()> {

        let ret = unsafe { CreateIoCompletionPort(fd, self.0, key, 0) };
        if ret.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }


    pub fn get_queued() {}

    pub fn get_queued_many() {}
    


}