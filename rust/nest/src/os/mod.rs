
mod unix;
mod win32;

#[cfg(any(target_os = "linux"))]
pub use self::unix::consts::os as consts;

#[cfg(any(target_os = "windows"))]
pub use self::win32::consts::os as consts;
