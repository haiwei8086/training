
mod unix;
mod win32;

#[cfg(any(target_os = "linux", target_os = "android",
          target_os = "macos", target_os = "ios",
          target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub use self::unix::consts::os as consts;

#[cfg(any(target_os = "windows"))]
pub use self::win32::consts::os as consts;
