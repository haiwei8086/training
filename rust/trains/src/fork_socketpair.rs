extern crate libc;

use libc::{c_int, c_void, size_t};
use std::{mem};
use std::os::unix::io::RawFd;

const AF_UNIX: c_int  = 1;
const SOCK_STREAM: c_int = 1;

extern {
    fn socketpair(domain: c_int, typ: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
}

pub fn run(){

    let (wfd, rfd) = createSocketpair().unwrap();

    for _ in 0..2 {
        let pid = unsafe { libc::fork() };

        if pid == 0 {
            worker(rfd);
            break;
        } else {
            master(wfd);
        }
    }
}

fn master(fd: RawFd) {

    write(fd, b"Master signal").unwrap();

    println!(
        "Master proccess. pid: {:?}, ppid: {:?}",
        unsafe { libc::getpid() },
        unsafe { libc::getppid() }
    );

    let mut r_buf = vec![0; 20];
    let r_len = read(fd, &mut r_buf).unwrap();
    r_buf.truncate(r_len);
    let str = unsafe { String::from_utf8_unchecked(r_buf) };
    println!("Master Read len: {:?}, buffer: {:?}, pid: {:?}", r_len, str, unsafe { libc::getpid() });
}

fn worker(fd: RawFd) {

    let mut r_buf = vec![0; 20];
    let r_len = read(fd, &mut r_buf).unwrap();
    r_buf.truncate(r_len);
    let str = unsafe { String::from_utf8_unchecked(r_buf) };
    println!("Worker Read len: {:?}, buffer: {:?}, pid: {:?}", r_len, str, unsafe { libc::getpid() });

    println!(
        "Worker proccess. pid: {:?}, ppid: {:?}",
        unsafe { libc::getpid() },
        unsafe { libc::getppid() }
    );

    write(fd, b"Worker signal").unwrap();
}

fn createSocketpair() -> Result<(RawFd, RawFd), usize> {
    let mut fds = [-1; 2];

    let sock = unsafe { socketpair(AF_UNIX, SOCK_STREAM, 0, fds.as_mut_ptr()) };

    if sock < 0 {
        return Err(sock as usize);
    }

    return Ok((fds[0], fds[1]));
}

fn read(fd: RawFd, buf: &mut [u8]) -> Result<usize, usize> {
    let res = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t) };

    if res < 0 {
        return Err(res as usize);
    }

    return Ok(res as usize);
}

fn write(fd: RawFd, buf: &[u8]) -> Result<usize, usize> {

    let res = unsafe { libc::write(fd, buf.as_ptr() as *const c_void, buf.len() as size_t) };

    if res < 0 {
        return Err(res as usize);
    }

    return Ok(res as usize);
}

#[test]
fn test_socketpair() {

    let (wfd, rfd) = createSocketpair().unwrap();

    let buf = b"Hello World!";
    let w_len = write(wfd, buf).unwrap();
    println!("Write len: {:?}", w_len);

    let mut r_buf = vec![0; 20];
    let r_len = read(rfd, &mut r_buf).unwrap();
    r_buf.truncate(r_len);
    let str = unsafe { String::from_utf8_unchecked(r_buf) };
    println!("Read len: {:?}, buffer: {:?}", r_len, str);
}
