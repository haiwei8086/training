extern crate libc;

mod efi;

fn main() {
    println!("Runing....");

    efi::run();

}
