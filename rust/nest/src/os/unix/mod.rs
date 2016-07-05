use libc;

use super::{NsResult, NsError};


pub fn fork() -> NsResult<i32> {
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(NsError::Unknow);
    }

    Ok(pid)
}
