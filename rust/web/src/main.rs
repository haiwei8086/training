extern crate libc;


use std::net;

fn main() {

    /*
    let ip = sys::socket::ip::IPv4Addr::any();
    println!("{:?}", ip.to_std());
    */
    let addr = net::Ipv4Addr::new(127, 0, 0, 1);
    let ptr = addr.as_inner();
    println!("{:?}", ptr);

    println!("Hello, world!");
}
