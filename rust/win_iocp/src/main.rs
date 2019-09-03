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
mod socket;
mod event;


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
    println!("from std addr: {}", InetAddr::from(std_addr));



    let mut ctx = context::Context::new();
    println!("Context PID: {}", ctx.pid);


    os::init(&mut ctx);
    println!("Context arguments: {:?}", ctx.arguments);


    if ctx.arguments.contains_key("-s") {
        if let Some(val) = ctx.arguments.get("-s") {
            process::signal_process(&ctx, val);
            return;
        }
    }

    
    os::save_pid(&ctx);

    let pid = os::read_pid(&ctx).unwrap();
    println!("Read pid: {}", pid);


    os::set_environment("win_iocp", "123");
    
    let env_val = os::get_environment("win_iocp");
    println!("Environment value: {}", env_val);


    process::single_process_cycle(&mut ctx);
    //process::master_process_cycle(&mut ctx);

}
