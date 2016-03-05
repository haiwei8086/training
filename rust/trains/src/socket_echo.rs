extern crate libc;

use std::net::TcpListener;
use std::thread;
use std::io::prelude::*;


pub fn run(){
    std_socket();
}


fn std_socket(){

    println!("Std socket: listening on 127.0.0.1:3000");
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    loop {
        match listener.accept() {
            Ok((mut stream, addr)) => {

                thread::spawn(move || {
                    println!("Std socket: remoter {:?}", addr);
                    println!("Std socket: stream {:?}", stream);

                    let mut buf = vec![0; 512];
                    stream.read(&mut buf);
                    println!("Data: {:?}", String::from_utf8_lossy(&buf));

                    stream.write(b"Hello world!").unwrap();
                });

            },
            Err(e) => println!("listener error: {:?}", e)
        }
    }

    println!("Std socket: drop listener");
    drop(listener);
}
