pub mod winapi {
    extern crate winapi;
    pub use self::winapi::*;
}

use std::net;

mod ffi;
mod consts;
mod ip;
mod sock_addr;
mod os;
mod context;
mod process;


use ip::IPAddrV4;
use sock_addr::InetAddr;



fn main() {
    println!("Win ICOP");

    let ipv4 = IPAddrV4::new(127, 0, 0, 0);
    
    println!("IPv4: {}", ipv4);


    let std_addr  = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let inet_addr = InetAddr::new(ip::IPAddr::V4(<IPAddrV4>::new(127, 0, 0, 1)), 8080);

    assert_eq!("127.0.0.1:8080".parse(), Ok(std_addr));
    assert_eq!(inet_addr, inet_addr);

    assert_eq!(InetAddr::from(std_addr), inet_addr);
    assert_eq!(std_addr, net::SocketAddr::from(inet_addr));

    println!("std addr: {}", std_addr);
    println!("inet_addr: {}", inet_addr);
    println!("from std addr: {}", InetAddr::from(std_addr));



    let mut ctx = context::Context::new();
    println!("Context PID: {}", ctx.pid);


    os::init();

    process::master_process_cycle(&mut ctx);

}
