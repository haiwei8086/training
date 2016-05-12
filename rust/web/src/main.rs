extern crate nest;

use nest::net::{ip};

fn main() {
    nest_test();

    println!("Hello, world!");
}

fn nest_test() {
    println!("Version: {:?}", nest::version());

    let ip = ip::IpAddr::V4(ip::Ipv4Addr::new(127, 0, 0, 1));

    println!("IP: {:?}", ip);
    println!("IP str: {:?}", ip.to_string());
}

/*
fn stain_test() {
    stain::version();

    let ip = ip::IpAddr::V4(ip::Ipv4Addr::new(127, 0, 0, 1));
    println!("ip v4: {:?}", ip);

    let addr = addr::InetAddr::new(ip, 5000);
    println!("addr v4: {:?}", addr.to_str());
}
*/
