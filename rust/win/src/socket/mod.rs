
mod consts;
mod ffi;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AddressFamily {
    Unix = consts::AF_UNIX,
    Inet = consts::AF_INET,
    Inet6 = consts::AF_INET6,
}


#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SockType {
    Stream = consts::SOCK_STREAM,
    Datagram = consts::SOCK_DGRAM,
    SeqPacket = consts::SOCK_SEQPACKET,
    Raw = consts::SOCK_RAW,
    Rdm = consts::SOCK_RDM,
}




pub fn init() {

    println!("Win Socket bind to Localhost: 9090");
}
