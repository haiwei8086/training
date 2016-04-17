
extern crate stain;

use stain::net::{ip, addr};

fn main() {
    stain::version();

    let ip = ip::IpAddr::V4(ip::Ipv4Addr::new(127, 0, 0, 1));
    println!("ip v4: {:?}", ip);

    let addr = addr::InetAddr::new(ip, 5000);
    println!("addr v4: {:?}", addr.to_str());

    println!("Hello, world!");
}
