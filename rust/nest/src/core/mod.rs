mod ip;
mod addr;
mod socketopts;

use libc::{self, c_int};
use std::os::unix::io::RawFd;

use super::{NsResult, NsError};

pub use self::consts::os::*;
pub use self::ip::*;
pub use self::addr::*;
pub use self::socketopts::*;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum NsSocketTypes {
    Stream = SOCK_STREAM,
    Datagrams = SOCK_DGRAM,
    Raw = SOCK_RAW,
    Rdm = SOCK_RDM,
    SeqPacket = SOCK_SEQPACKET,
}
