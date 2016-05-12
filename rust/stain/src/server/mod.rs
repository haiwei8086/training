
use std::net::SocketAddr;
use std::str::FromStr;

pub struct Server {

}

impl Server {

    pub fn new() -> Server {
        println!("Server new");
        Server {}
    }

    pub fn listen(addr: &str) {
        let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

        println!("addr: {:?}, IP: {}, Port: {}", addr, addr.ip(), addr.port());
    }

}
