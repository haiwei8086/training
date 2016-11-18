
use std;

pub fn run() {

    println!("Current directory: {}", std::env::current_dir().unwrap().display());
    println!("Current exe: {:?}", std::env::current_exe());

    for argument in std::env::args() {
        println!("{}", argument);
    }
}
