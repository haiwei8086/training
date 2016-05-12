/* features */
#![crate_name = "stain"]

extern crate libc;

pub mod net;
pub mod server;


pub fn version() {
    println!("Rust Stain Version: 0.1.0");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
