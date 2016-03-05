extern crate nix;

use std::net::SocketAddr;
use std::str::FromStr;
use self::nix::sys::socket;
use self::nix::sys::socket::{InetAddr, AddressFamily, SockType, SockAddr};
use self::nix::sys::socket::SOCK_NONBLOCK;
use self::nix::sys::epoll;
use self::nix::sys::epoll::{EpollEvent, EPOLLIN};

pub fn run(){
    println!("Libc socket epoll");
    let actual: SocketAddr = FromStr::from_str("127.0.0.1:3000").unwrap();
    let socket_addr = SockAddr::new_inet(InetAddr::from_std(&actual));

    let listerfd = socket::socket(AddressFamily::Inet, SockType::Stream, SOCK_NONBLOCK).unwrap();

    if listerfd == -1 {
        println!("Create socket failed!");
        return;
    }
    match socket::bind(listerfd, &socket_addr) {
        Ok(_) => println!("Socket bind successfully."),
        Err(_) => println!("Socket bind failed!")
    }

    let epollfd = epoll::epoll_create().unwrap();
    /*
    match epollfd {
        Ok(_) => println!("Epoll create successfully"),
        Err(_) => println!("Epoll create failed")
    }
    */

    /*
    let event_item = EpollEvent{events: EPOLLIN, data: listerfd };
    let mut events: Vec<EpollEvent> = Vec::new();

    match epoll::epoll_ctl(epollfd, epoll::EpollOp::EpollCtlAdd, listerfd, &event_item) {
        Ok(_) => println!("Add event to epoll."),
        Err(_) => println!("Add event to epoll failed.")
    }

    for _ in 0..10 {
        let num = epoll::epoll_wait(epollfd, &mut events, 2000).unwrap();

        println!("event type:{:?}", events);

        println!("epoll num: {:?}", num);
    }
    */

    println!("Libc socket epoll end");
}
