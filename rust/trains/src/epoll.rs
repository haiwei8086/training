use super::libc::{c_int};
use std::os::unix::io::RawFd;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpollEvent {
    pub events: u32,
    pub data: u64
}


extern "C" {
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
    fn epoll_wait(epfd: c_int, event: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
}

pub mod ctl_op {
    /// Register the target file descriptor fd on the epoll instance
    /// referred to by the file descriptor epfd and associate the
    /// event with the internal file linked to fd.
    pub const ADD: u32 = 1;
    /// Change the event event associated with the target file descriptor fd.
    pub const MOD: u32 = 2;
    /// Remove (deregister) the target file descriptor fd from the epoll
    /// instance referred to by epfd. The event is ignored and can be NULL.
    pub const DEL: u32 = 3;
}

pub mod event_type {
    /// The associated file is available for read(2) operations.
    pub const EPOLLIN:          u32 = 0x001;
    /// The associated file is available for write(2) operations.
    pub const EPOLLOUT:         u32 = 0x004;
    /// Stream socket peer closed connection, or shut down writing
    /// half of connection.
    pub const EPOLLRDHUP:       u32 = 0x2000;
    /// There is urgent data available for read(2) operations.
    pub const EPOLLPRI:         u32 = 0x002;
    /// Error condition happened on the associated file descriptor.
    /// epoll_wait(2) will always wait for this event; it is not
    /// necessary to set it in events.
    pub const EPOLLERR:         u32 = 0x008;
    /// Hang up happened on the associated file descriptor. epoll_wait(2)
    /// will always wait for this event; it is not necessary to set it
    /// in events.
    pub const EPOLLHUP:         u32 = 0x010;
    /// Sets the Edge Triggered behavior for the associated file descriptor.
    /// The default behavior for epoll is Level Triggered.
    pub const EPOLLET:          u32 = (1 << 31);
    /// Sets the one-shot behavior for the associated file descriptor.
    /// This means that after an event is pulled out with epoll_wait(2)
    /// the associated file descriptor is internally disabled and no other
    /// events will be reported by the epoll interface. The user must call
    /// epoll_ctl() with EPOLL_CTL_MOD to rearm the file descriptor with a
    /// new event mask.
    pub const EPOLLONESHOT:     u32 = (1 << 30);
    /// If EPOLLONESHOT and EPOLLET are clear and the process has the
    /// CAP_BLOCK_SUSPEND capability, ensure that the system does not
    /// enter "suspend" or "hibernate" while this event is pending or
    /// being processed.  The event is considered as being "processed"
    /// from the time when it is returned by a call to epoll_wait(2)
    /// until the next call to epoll_wait(2) on the same epoll(7) file
    /// descriptor, the closure of that file descriptor, the removal
    /// of the event file descriptor with EPOLL_CTL_DEL, or the
    /// clearing of EPOLLWAKEUP for the event file descriptor with
    /// EPOLL_CTL_MOD.
    pub const EPOLLWAKEUP:      u32 = (1 << 29);
}


#[inline]
pub fn create1(flags: u32) -> Result<RawFd, usize> {
    let epoll_fd;
    unsafe {
        epoll_fd = epoll_create1(flags as c_int);
    }

    if epoll_fd < 0 {
        return Err(epoll_fd as usize);
    }

    Ok(epoll_fd)
}

/// Calls epoll_ctl(2) with supplied params
#[inline]
pub fn ctl(epoll_fd: RawFd, op: u32, socket_fd: RawFd, event: &mut EpollEvent) -> Result<(), usize> {
    let x;
    unsafe {
        x = epoll_ctl(epoll_fd as c_int,
            op as c_int,
            socket_fd as c_int,
            event as *mut EpollEvent);
    }

    if x < 0 {
        return Err(x as usize);
    }

    Ok(())
}

/// Calls epoll_wait(1) with supplied params
#[inline]
pub fn wait(epoll_fd: RawFd, events: &mut [EpollEvent], timeout: i32) -> Result<u32, usize> {

    let num_fds_ready;
    unsafe {
        num_fds_ready = epoll_wait(epoll_fd as c_int,
            events.as_mut_ptr(),
            events.len() as c_int,
            timeout as c_int);
    }

    if num_fds_ready < 0 {
        return Err(num_fds_ready as usize);
    }

    Ok(num_fds_ready as u32)
}
