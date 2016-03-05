extern crate libc;

mod addr;
mod epoll;
//mod fork_socketpair;
mod epoll_socket;

fn main() {
    println!("Running...");

    //fork_socketpair::run();
    epoll_socket::run();
}
