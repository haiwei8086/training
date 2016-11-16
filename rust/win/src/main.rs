
extern crate rustc_serialize;

 // mod win_form;
// mod winiocp;
// mod json;
mod socket;


fn main() {

    // win_form::run();
    // winiocp::run();
    // json::run();
    socket::run();

    println!("Hello, world!");
}
