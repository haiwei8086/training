extern crate libc;

mod addr;
mod web_server;

fn main() {
    println!("Running...");

    web_server::run();
}
